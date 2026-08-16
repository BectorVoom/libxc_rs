//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 258/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk258(t769: f64, t896: f64, t334: f64, t317: f64, t19: f64, t328: f64, t275: f64, t308: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t897 = t896 * t769;
    let t906 = t334 * t334;
    let t907 = 1.0_f64 / t906;
    let t908 = t317 * t907;
    let t909 = t19 * t328;
    let t910 = t308 * t275;
    let t911 = 1.0_f64 / t910;
    (t897, t906, t907, t908, t909, t910, t911)
}
