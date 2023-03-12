use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Default, Copy, Clone)]
pub struct Mat3(pub [[f32; 3]; 3]);

impl Mat3 {
    // pub fn transform(pos: [f32; 3], rotation: f32, scale: [f32; 3]) -> Self {}
}

impl Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self([
            [self.0[0][0] * rhs, self.0[0][1] * rhs, self.0[0][2] * rhs],
            [self.0[1][0] * rhs, self.0[1][1] * rhs, self.0[1][2] * rhs],
            [self.0[2][0] * rhs, self.0[2][1] * rhs, self.0[2][2] * rhs],
        ])
    }
}

impl MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::mul(*self, rhs);
    }
}

impl Div<f32> for Mat3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::mul(self, 1.0 / rhs)
    }
}

impl DivAssign<f32> for Mat3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self::div(*self, rhs);
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self([
            [
                self.0[0][0] * rhs.0[0][0]
                    + self.0[0][1] * rhs.0[1][0]
                    + self.0[0][2] * rhs.0[2][0],
                self.0[0][0] * rhs.0[0][1]
                    + self.0[0][1] * rhs.0[1][1]
                    + self.0[0][2] * rhs.0[2][1],
                self.0[0][0] * rhs.0[0][2]
                    + self.0[0][1] * rhs.0[1][2]
                    + self.0[0][2] * rhs.0[2][2],
            ],
            [
                self.0[1][0] * rhs.0[0][0]
                    + self.0[1][1] * rhs.0[1][0]
                    + self.0[1][2] * rhs.0[2][0],
                self.0[1][0] * rhs.0[1][1]
                    + self.0[1][1] * rhs.0[1][1]
                    + self.0[1][2] * rhs.0[2][1],
                self.0[1][0] * rhs.0[1][2]
                    + self.0[1][1] * rhs.0[1][2]
                    + self.0[1][2] * rhs.0[2][2],
            ],
            [
                self.0[2][0] * rhs.0[0][0]
                    + self.0[2][1] * rhs.0[1][0]
                    + self.0[2][2] * rhs.0[2][0],
                self.0[2][0] * rhs.0[0][1]
                    + self.0[2][1] * rhs.0[1][1]
                    + self.0[2][2] * rhs.0[2][1],
                self.0[2][0] * rhs.0[0][2]
                    + self.0[2][1] * rhs.0[1][2]
                    + self.0[2][2] * rhs.0[2][2],
            ],
        ])
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::mul(*self, rhs);
    }
}

impl Mul<[f32; 3]> for Mat3 {
    type Output = [f32; 3];

    fn mul(self, rhs: [f32; 3]) -> [f32; 3] {
        [
            [
                self.0[0][0] * rhs[0], 
                self.0[0][0] * rhs[0], 
                self.0[0][0] * rhs[0], 
            ],
            [
            ],
            [
            ],
        ]
    }
}

impl MulAssign<[f32; 3]> for Mat3 {
    fn mul_assign(&mut self, rhs: [f32; 3]) {
        *self = Self::mul(*self, rhs);
    }
}