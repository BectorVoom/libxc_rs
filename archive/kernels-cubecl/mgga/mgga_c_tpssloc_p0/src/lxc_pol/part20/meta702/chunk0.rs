//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2671/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2671<F: Float>(t39611: F, t39620: F, t39628: F, t39630: F, t39632: F, t39634: F, t39636: F, t39642: F, t39644: F, t5154: F, t9722: F, t39659: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t54442 = F::cast_from(360.0_f64) * t39611;
    let t54443 = F::cast_from(3.0_f64) * t39620;
    let t54444 = F::cast_from(60.0_f64) * t39628;
    let t54445 = F::cast_from(4.0_f64) * t39630;
    let t54446 = F::cast_from(4.0_f64) * t39632;
    let t54447 = F::cast_from(48.0_f64) * t39634;
    let t54448 = F::cast_from(72.0_f64) * t39636;
    let t54449 = F::cast_from(3.0_f64) * t39642;
    let t54450 = F::cast_from(24.0_f64) * t39644;
    let t54451 = t5154 * t9722;
    let t54452 = F::cast_from(0.10389515463408878255e3_f64) * t54451;
    let t54453 = F::cast_from(96.0_f64) * t39659;
    (t54442, t54443, t54444, t54445, t54446, t54447, t54448, t54449, t54450, t54452, t54453)
}
