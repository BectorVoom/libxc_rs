//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 247/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk247(t45: f64, t57: f64, t190: f64, t606: f64, t706: f64, t78: f64, t81: f64, t150: f64, t169: f64, t164: f64, t687: f64, t689: f64, t693: f64, t698: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t707 = t190 * t606;
    let t709 = 4.0_f64 * t706 * t707;
    let t712 = piecewise3(t151, 0.0_f64, 4.0_f64 / 3.0_f64 * t78 * t606);
    let t715 = piecewise3(t155, 0.0_f64, -4.0_f64 / 3.0_f64 * t81 * t606);
    let t716 = t712 + t715;
    let t717 = t150 * t716;
    let t718 = t717 * t190;
    let t722 = t169 * t169;
    let t723 = 1.0_f64 / t722;
    let t724 = t164 * t723;
    let t729 = -0.1176575e1_f64 * t687 - 0.516475e0_f64 * t689 - 0.2103875e0_f64 * t693 - 0.104195e0_f64 * t698;
    (t707, t709, t716, t717, t718, t722, t723, t724, t729)
}
