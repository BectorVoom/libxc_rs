//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1354/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1354(t136: f64, t2826: f64, t76608: f64, t76612: f64, t908: f64, t76616: f64, t76620: f64, t43002: f64, t48103: f64, t60168: f64, t60173: f64, t60204: f64, t68452: f64, t68454: f64, t76903: f64) -> (f64, f64, f64, f64, f64) {
    let t76906 = t136 * t2826 * t76608;
    let t76909 = t136 * t908 * t76612;
    let t76912 = t136 * t908 * t76616;
    let t76915 = t136 * t908 * t76620;
    let t76922 = -4.0_f64 / 3.0_f64 * t76903 + 2.0_f64 / 9.0_f64 * t76906 - 4.0_f64 * t76909 + 6.0_f64 * t76912 - t76915 - 20.0_f64 / 9.0_f64 * t60168 + 10.0_f64 / 9.0_f64 * t60173 + 8.0_f64 / 3.0_f64 * t68452 - t43002 - 4.0_f64 / 9.0_f64 * t68454 - 160.0_f64 / 81.0_f64 * t48103 + 10.0_f64 / 27.0_f64 * t60204;
    (t76906, t76909, t76912, t76915, t76922)
}
