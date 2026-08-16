//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3394/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3394(t41307: f64, t41361: f64, t41363: f64, t51967: f64, t51973: f64, t51978: f64, t63299: f64, t63304: f64, t63308: f64, t63311: f64, t63315: f64, t63320: f64, t63325: f64, t63328: f64, t63332: f64) -> f64 {
    let t63731 = 0.60385e0_f64 * t63299 + 0.40256666666666666666e1_f64 * t63304 - 0.72462e1_f64 * t63308 + t41307 - 0.99342e0_f64 * t63311 + 0.198684e1_f64 * t63315 + 0.20128333333333333334e0_f64 * t51967 - 0.53675555555555555558e0_f64 * t51973 + 0.62621481481481481484e0_f64 * t51978 + 0.11038e0_f64 * t63320 + 0.62621481481481481482e0_f64 * t41361 + 0.26837777777777777778e0_f64 * t41363 - 0.13418888888888888889e1_f64 * t63325 + 0.48307999999999999999e1_f64 * t63328 + 0.33114e0_f64 * t63332;
    t63731
}
