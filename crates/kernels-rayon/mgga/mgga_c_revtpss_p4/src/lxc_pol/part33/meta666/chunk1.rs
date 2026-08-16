//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2180/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2180(t22299: f64, t26028: f64, t22093: f64, t22098: f64, t108531: f64, t108533: f64, t108535: f64, t108537: f64, t108539: f64, t108541: f64, t108543: f64, t98129: f64, t98131: f64) -> f64 {
    let t108545 = t26028 * t22299;
    let t108547 = t26028 * t22093;
    let t108549 = t26028 * t22098;
    let t108551 = -0.34299214494455789578e-2_f64 * t108531 + 0.17149607247227894789e-2_f64 * t108533 - 0.68598428988911579156e-2_f64 * t108535 - t98129 + t98131 + 7.0_f64 / 144.0_f64 * t108537 - 7.0_f64 / 48.0_f64 * t108539 - t108541 / 48.0_f64 - 0.17149607247227894789e-1_f64 * t108543 + 0.34299214494455789578e-2_f64 * t108545 + 0.34299214494455789578e-2_f64 * t108547 + 0.34299214494455789578e-2_f64 * t108549;
    t108551
}
