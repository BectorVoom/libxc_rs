//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2155/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2155(t20020: f64, t7117: f64, t100275: f64, t100289: f64, t18904: f64, t18913: f64, t18937: f64, t18942: f64, t19861: f64, t20040: f64, t25495: f64, t27526: f64, t27527: f64, t27531: f64, t53321: f64, t6278: f64, t93752: f64, t93801: f64) -> f64 {
    let t107140 = t7117 * t20020;
    let t107144 = t100275 + t100289 - t27526 * t27527 * t18942 / 144.0_f64 + t27526 * t27531 * t18937 / 216.0_f64 + t27526 * t27531 * t18913 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t27526 * t53321 * t18904 - 0.57165357490759649296e-3_f64 * t93752 * t19861 - 0.57165357490759649296e-3_f64 * t93752 * t20040 - 0.95275595817932748827e-4_f64 * t93801 - 0.28582678745379824648e-3_f64 * t107140 + 0.22866142996303859718e-2_f64 * t25495 * t6278;
    t107144
}
