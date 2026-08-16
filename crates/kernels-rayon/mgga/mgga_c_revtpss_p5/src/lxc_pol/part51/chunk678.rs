//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 678/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk678(t2035: f64, t7235: f64, t2033: f64, t531: f64, t1353: f64, t1450: f64, t2014: f64, t2022: f64, t212: f64, t1358: f64, t689: f64, t2023: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7236 = t7235 * t2035;
    let t7237 = t531 * t2033;
    let t7238 = t1450 * t1353;
    let t7239 = t7237 * t7238;
    let t7241 = 3.0_f64 * t2014 * t7239;
    let t7242 = t212 * t2022;
    let t7243 = t7242 * t1358;
    let t7245 = 0.54878743191129263322e-2_f64 * t689 * t7243;
    let t7246 = t786 * t2023;
    (t7236, t7237, t7238, t7239, t7241, t7242, t7243, t7245, t7246)
}
