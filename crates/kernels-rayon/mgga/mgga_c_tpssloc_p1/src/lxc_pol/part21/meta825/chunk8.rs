//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2908/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2908(t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t60186: f64, t60189: f64, t60192: f64, t60194: f64, t60197: f64, t60200: f64, t60202: f64, t60204: f64, t60207: f64) -> f64 {
    let t60634 = 0.6311625e0_f64 * t60186 + 0.250068e1_f64 * t60189 + 0.123954e2_f64 * t59661 + 0.83356000000000000001e0_f64 * t60192 - 0.55570666666666666667e0_f64 * t60194 - 0.62517e0_f64 * t60197 + 0.41678e0_f64 * t60200 - 0.27785333333333333334e0_f64 * t60202 - 0.38590740740740740742e-1_f64 * t60204 - 0.69463333333333333334e-1_f64 * t60207 - 0.68863333333333333333e0_f64 * t59663 + 0.22954444444444444444e0_f64 * t59665 - 0.68863333333333333334e0_f64 * t59670 - 0.34431666666666666667e0_f64 * t59674 - 0.68863333333333333334e0_f64 * t59678;
    t60634
}
