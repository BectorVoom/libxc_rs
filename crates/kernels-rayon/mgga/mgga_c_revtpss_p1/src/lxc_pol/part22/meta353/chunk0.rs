//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1851/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1851(t1053: f64, t3204: f64, t127: f64, t3218: f64, t371: f64, t1025: f64, t1058: f64, t3191: f64, t1021: f64, t3201: f64, t3231: f64, t1054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11947 = t3204 * t1053;
    let t11951 = t371 * t127 * t3218;
    let t11952 = t1025 * t11951;
    let t11954 = t3191 * t1058;
    let t11956 = t1021 * t3201;
    let t11965 = t3231 * t1058;
    let t11967 = t1054 * t3201;
    (t11947, t11951, t11952, t11954, t11956, t11965, t11967)
}
