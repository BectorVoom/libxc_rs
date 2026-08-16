//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 847/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk847(t3154: f64, t357: f64, t11249: f64, t905: f64, t3182: f64, t828: f64, t3109: f64, t126: f64, t3181: f64, t221: f64, t346: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11631 = t3154 * t357;
    let t11632 = t11249 * t11631;
    let t11660 = t3154 * t905;
    let t11703 = t828 * t3182;
    let t11710 = t828 * t3109;
    let t11725 = t126 * t3181;
    let t11735 = t221 * t68 * t346;
    (t11631, t11632, t11660, t11703, t11710, t11725, t11735)
}
