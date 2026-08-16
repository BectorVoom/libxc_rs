//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1198/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1198(t1358: f64, t7242: f64, t689: f64, t2023: f64, t786: f64, t1364: f64, t533: f64, t7021: f64, t816: f64, t1941: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7243 = t7242 * t1358;
    let t7245 = 0.54878743191129263322e-2_f64 * t689 * t7243;
    let t7246 = t786 * t2023;
    let t7248 = 0.9757440539382783019e-2_f64 * t7246 * t1364;
    let t7250 = t7021 * t533 * t816;
    let t7251 = 7.0_f64 / 288.0_f64 * t7250;
    let t7252 = t1941 * t540;
    (t7243, t7245, t7246, t7248, t7251, t7252)
}
