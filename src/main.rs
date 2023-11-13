fn main() {
    let array: [i32; 5] = [0; 5];
    let boolean: bool = true;
    let character: char = 'A';
    let float32: f32 = 2.5;
    let new_string: &'static str = "my name";

    println!("array  = {:?}, size = {}", array, std::mem::size_of_val(&array));
    println!("boolean = {}, size = {}", boolean, std::mem::size_of_val(&boolean));
    println!("character = {}, size = {}", character, std::mem::size_of_val(&character));
    println!("float32 = {}, size = {}", float32, std::mem::size_of_val(&float32));
    println!("string = {}, size = {}", new_string, std::mem::size_of_val(new_string));

    println!("\nSum 2.5 + 2 = {}", sum(2.5f32, 2f32));
    println!("Subtract 20 + 4.65 = {}", subtraction(20f32, 4.65));
    println!("Multiply 20 + 4.65 = {}", multiply(20f32, 4.65));
    println!("Divide 20 + 4.65 = {:.2}", divide(20f32, 4.65));
}

fn sum(num1: f32, num2: f32) -> f32 {
    num1 + num2
}

fn subtraction(num1: f32, num2: f32) -> f32 {
    num1 - num2
}

fn multiply(num1: f32, num2: f32) -> f32 {
    num1 * num2
}

fn divide(num1: f32, num2: f32) -> f32 {
    num1 / num2
}