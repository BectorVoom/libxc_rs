//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 698/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk698(t115: f64, t138: f64, t5: f64, t529: f64, t6: f64, t23: f64, t864: f64, t212: f64, t2269: f64) -> (f64, f64, f64, f64) {
    let t7128 = t138 * t115;
    let t7129 = t7128 * t5;
    let t7205 = t6 * t529;
    let t7212 = t23 * t864;
    let t7253 = 1.0_f64 / t212 / t2269;
    (t7129, t7205, t7212, t7253)
}
