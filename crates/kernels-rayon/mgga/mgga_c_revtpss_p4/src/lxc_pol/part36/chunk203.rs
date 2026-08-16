//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 203/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk203(t169: f64, t164: f64, t687: f64, t689: f64, t693: f64, t698: f64, t172: f64) -> (f64, f64, f64, f64, f64) {
    let t722 = t169 * t169;
    let t723 = 1.0_f64 / t722;
    let t724 = t164 * t723;
    let t729 = -0.1176575e1_f64 * t687 - 0.516475e0_f64 * t689 - 0.2103875e0_f64 * t693 - 0.104195e0_f64 * t698;
    let t730 = 1.0_f64 / t172;
    (t722, t723, t724, t729, t730)
}
