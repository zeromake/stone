extern crate regex;
use regex::Regex;

fn main() {
    let arr = [
        // 注释
        r"\s*((//.*)",
        // 八进制
        r"(0[0-7]+)",
        // hex
        r"(0x[0-9a-fA-F]+)",
        // flost
        r"([0-9]+.[0-9]+)",
        // int
        r"([0-9]+)",
        // string
        r#"("(\\"|\\|\n|[^"])*")"#,
        // char
        r#"('(\\'|\\|\n|[^'])*')"#,
        // variable
        r"[A-z_a-z][A-z_a-z0-9]*",
        r"\.",
        // 
        r"\{|\}|\(|\)",
        // 计算语句
        r"=|==|>=|<=|>|<|&&|\|\||&|\|)?",
    ];
    let regex_str = &arr.join("|");
    let regex_pat = Regex::new(regex_str).unwrap();
    let code_str = r#"let test = 'ff'"#;
    for caps in regex_pat.captures_iter(code_str) {
        let cap = caps.get(1);
        if cap != None {
            println!("{}", cap.unwrap().as_str());
        }
    }
}
