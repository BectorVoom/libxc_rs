//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2906/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2906(t41656: f64, t41658: f64, t41675: f64, t41684: f64, t41863: f64, t41870: f64, t41872: f64, t47738: f64, t48103: f64, t48116: f64, t59655: f64, t60091: f64, t60150: f64, t60153: f64, t60156: f64) -> f64 {
    let t60601 = 0.20659e1_f64 * t47738 + 0.61745185185185185187e0_f64 * t48103 - 0.22954444444444444444e0_f64 * t41656 - 0.15302962962962962963e0_f64 * t41658 + 0.45908888888888888888e0_f64 * t41675 + 0.10712074074074074074e1_f64 * t41684 + 0.61745185185185185184e0_f64 * t41863 - 0.11577222222222222222e0_f64 * t41870 - 0.3859074074074074074e-1_f64 * t41872 + 0.61745185185185185187e-1_f64 * t48116 - 0.250068e1_f64 * t60091 - 0.123954e2_f64 * t59655 + 0.6311625e0_f64 * t60150 + 0.83356000000000000001e0_f64 * t60153 - 0.18523555555555555556e0_f64 * t60156;
    t60601
}
