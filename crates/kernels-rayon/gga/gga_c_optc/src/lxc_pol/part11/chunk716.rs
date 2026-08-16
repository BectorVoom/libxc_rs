//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 716/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk716(t2746: f64, t298: f64, t301: f64, t305: f64, t8113: f64, t19: f64, t7380: f64, t123: f64, t3906: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8124 = 1.0_f64 / t2746 / t298;
    let t8125 = t8124 * t301;
    let t8126 = t8125 * t305;
    let t8127 = t8126 * t8113;
    let t8128 = t7380 * t19;
    let t8129 = t8128 * t123;
    let t8134 = t3906 * t8113;
    (t8124, t8125, t8126, t8127, t8128, t8129, t8134)
}
