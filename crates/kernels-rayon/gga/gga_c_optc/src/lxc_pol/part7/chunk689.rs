//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 689/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk689(t6461: f64, t6523: f64, t60: f64, t40: f64, t1948: f64, t729: f64, t108: f64, t176: f64, t203: f64, t47: f64, t768: f64, t1885: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6524 = t6461 + t6523;
    let t6525 = t60 * t6524;
    let t6526 = t40 * t6525;
    let t6527 = t729 * t1948;
    let t6529 = t176 * t6527 * t108;
    let t6530 = t6529 * t203;
    let t6533 = 1.0_f64 / t47 / t768;
    let t6534 = t1885 * t549;
    (t6524, t6525, t6526, t6529, t6530, t6533, t6534)
}
