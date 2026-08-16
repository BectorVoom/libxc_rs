//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1140/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1140(t11940: f64, t366: f64, t11202: f64, t373: f64, t371: f64, t372: f64, t1053: f64, t3204: f64, t127: f64, t3218: f64, t1025: f64, t1058: f64, t3191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11941 = t11940 * t366;
    let t11942 = t373 * t11202;
    let t11944 = t371 * t372 * t11942;
    let t11947 = t3204 * t1053;
    let t11951 = t371 * t127 * t3218;
    let t11952 = t1025 * t11951;
    let t11954 = t3191 * t1058;
    (t11941, t11942, t11944, t11947, t11951, t11952, t11954)
}
