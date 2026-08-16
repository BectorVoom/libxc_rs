//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1314/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1314<F: Float>(t2374: F, t9467: F, t2749: F, t2752: F, t702: F, t9454: F, t2411: F) -> (F, F, F, F) {
    let t9469 = F::cast_from(0.21687162600603479684e-1_f64) * t2374 * t9467;
    let t9470 = t2749 * t2752;
    let t9474 = t9454 * t702;
    let t9476 = F::cast_from(6.0_f64) * t2411 * t9474;
    (t9469, t9470, t9474, t9476)
}
