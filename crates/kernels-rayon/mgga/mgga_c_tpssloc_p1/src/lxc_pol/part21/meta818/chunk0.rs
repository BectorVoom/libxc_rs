//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2881/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2881(t2815: f64, t60160: f64, t136: f64, t59659: f64, t908: f64, t17246: f64, t699: f64, t17249: f64, t59763: f64, t59767: f64, t17252: f64, t2403: f64, t5717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t60186 = t2815 * t60160;
    let t60189 = t136 * t908 * t59659;
    let t60192 = t699 * t17246;
    let t60194 = t699 * t17249;
    let t60197 = t136 * t908 * t59763;
    let t60200 = t136 * t908 * t59767;
    let t60202 = t699 * t17252;
    let t60204 = t2403 * t5717;
    (t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204)
}
