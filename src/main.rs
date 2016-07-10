
fn main() {
    println!("hello, world!");

    let trie =  HashMap<int, HashMap<char, String>>::new(); 

    let word = String::from("hello");

    for letter in word.chars()  {
        println!("{}", letter)
    }
}
