//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3417/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3417(t41361: f64, t41363: f64, t41690: f64, t51967: f64, t51973: f64, t51978: f64, t63299: f64, t63304: f64, t63308: f64, t63311: f64, t63315: f64, t63320: f64, t63325: f64, t63328: f64, t63332: f64) -> f64 {
    let t64228 = 0.103295e1_f64 * t63299 + 0.68863333333333333334e1_f64 * t63304 - 0.123954e2_f64 * t63308 + t41690 - 0.125034e1_f64 * t63311 + 0.250068e1_f64 * t63315 + 0.34431666666666666666e0_f64 * t51967 - 0.91817777777777777776e0_f64 * t51973 + 0.10712074074074074074e1_f64 * t51978 + 0.13892666666666666667e0_f64 * t63320 + 0.10712074074074074074e1_f64 * t41361 + 0.45908888888888888888e0_f64 * t41363 - 0.22954444444444444444e1_f64 * t63325 + 0.82636000000000000001e1_f64 * t63328 + 0.41678e0_f64 * t63332;
    t64228
}
