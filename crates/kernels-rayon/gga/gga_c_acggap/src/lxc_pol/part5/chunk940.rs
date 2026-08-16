//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 940/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk940(t14255: f64, t317: f64, t863: f64, t3883: f64, t852: f64, t13484: f64, t13487: f64, t180: f64, t14401: f64, t323: f64, t1210: f64, t851: f64) -> (f64, f64, f64, f64, f64) {
    let t14620 = 0.39512695097613069591e1_f64 * t863 * t317 * t14255;
    let t14621 = t852 * t3883;
    let t14626 = 0.15805078039045227836e2_f64 * t13484 * t180 * t317 * t13487;
    let t14640 = 0.26341796731742046395e1_f64 * t14401 * t180 * t323;
    let t14642 = t851 * t1210 * t323;
    (t14620, t14621, t14626, t14640, t14642)
}
