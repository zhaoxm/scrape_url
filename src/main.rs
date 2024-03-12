use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let output = &args[2];

    println!("url is {}", url);
    println!("output is {}", output);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let md = html2md::parse_html(&body);

    fs::write(output, md).unwrap();
}

