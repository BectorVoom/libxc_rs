//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2670/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2670<F: Float>(t39549: F, t39563: F, t39570: F, t39585: F, t39590: F, t39593: F, t39595: F, t54427: F, t54429: F, t54430: F, t54431: F, t54433: F, t54435: F, t54436: F, t54437: F, t54438: F, t54439: F) -> F {
    let t54440 = -t54427 + t39549 + t39563 + t54429 + t39570 + t54430 + t54431 - t39585 + t39590 - t39593 + t39595 + t54433 - t54435 + t54436 - t54437 + t54438 + t54439;
    t54440
}
