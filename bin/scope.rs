fn main()
{
    println!("-------SCOPE ANALYZER-------");
    let x = 10;
    println!("Original Value : {x}");
    let x = x + 15;
    println!("After Shadowing : {x}");
    let x = x + 25;
    println!("After Second Shadowing : {x}");
    let mut count = 5;
    println!("Mutable Counter : {count}");
    count = count + 10;
    println!("After Modification : {count}");
    let y = 50;
    {
        let y = y*2;
        println!("Inside block : {y}");
    }
    println!("Outside block : {y}");
}
