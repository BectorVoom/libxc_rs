//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1354/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1354<F: Float>(t136: F, t2826: F, t76608: F, t76612: F, t908: F, t76616: F, t76620: F, t43002: F, t48103: F, t60168: F, t60173: F, t60204: F, t68452: F, t68454: F, t76903: F) -> (F, F, F, F, F) {
    let t76906 = t136 * t2826 * t76608;
    let t76909 = t136 * t908 * t76612;
    let t76912 = t136 * t908 * t76616;
    let t76915 = t136 * t908 * t76620;
    let t76922 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76903 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t76906 - F::cast_from(4.0_f64) * t76909 + F::cast_from(6.0_f64) * t76912 - t76915 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t60168 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t60173 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t68452 - t43002 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t68454 - F::cast_from(160.0_f64) / F::cast_from(81.0_f64) * t48103 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t60204;
    (t76906, t76909, t76912, t76915, t76922)
}
