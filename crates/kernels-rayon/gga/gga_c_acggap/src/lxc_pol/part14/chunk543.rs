//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 543/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk543(t441: f64, t851: f64, t323: f64, t852: f64, t872: f64, t3101: f64, t317: f64, t316: f64, t3044: f64, t863: f64, t463: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3901 = t851 * t441;
    let t3902 = t3901 * t323;
    let t3906 = t852 * t872;
    let t3912 = t317 * t3101;
    let t3914 = 0.65854491829355115987e0_f64 * t316 * t3912;
    let t3915 = t317 * t3044;
    let t3917 = 0.39512695097613069591e1_f64 * t863 * t3915;
    let t3918 = t864 * t463;
    (t3902, t3906, t3912, t3914, t3915, t3917, t3918)
}
