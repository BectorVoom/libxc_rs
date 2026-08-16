//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 693/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk693(t2014: f64, t7239: f64, t2022: f64, t212: f64, t1358: f64, t689: f64, t2023: f64, t786: f64, t1364: f64, t533: f64, t7021: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7241 = 3.0_f64 * t2014 * t7239;
    let t7242 = t212 * t2022;
    let t7243 = t7242 * t1358;
    let t7245 = 0.54878743191129263322e-2_f64 * t689 * t7243;
    let t7246 = t786 * t2023;
    let t7248 = 0.9757440539382783019e-2_f64 * t7246 * t1364;
    let t7250 = t7021 * t533 * t816;
    (t7241, t7242, t7243, t7245, t7246, t7248, t7250)
}
