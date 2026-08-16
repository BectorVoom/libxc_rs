//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2624/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2624(t48287: f64, t39807: f64, t39813: f64, t47059: f64, t47063: f64, t47067: f64, t47070: f64, t47072: f64, t47076: f64, t48275: f64, t48278: f64, t48279: f64, t48281: f64, t48283: f64, t48284: f64, t48286: f64) -> (f64, f64) {
    let t48288 = 24.0_f64 * t48287;
    let t48289 = t47059 + t48275 + t39807 - t39813 + t47063 + t47067 - t48278 - t47070 - t47072 + t48279 - t48281 - t48283 - t47076 - t48284 + t48286 + t48288;
    (t48288, t48289)
}
