//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1094/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1094(t11513: f64, t7259: f64, t7944: f64, t11822: f64, t7511: f64, t11825: f64, t3708: f64, t9906: f64, t3330: f64, t10058: f64, t11784: f64, t10047: f64, t11387: f64, t3402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33576 = t7259 * t11513 * t7944;
    let t33578 = t11822 * t7511;
    let t33580 = t11825 * t7511;
    let t33582 = t9906 * t3708;
    let t33583 = t33582 * t3330;
    let t33585 = t11784 * t10058;
    let t33588 = t3402 * t11387 * t10047;
    (t33576, t33578, t33580, t33582, t33583, t33585, t33588)
}
