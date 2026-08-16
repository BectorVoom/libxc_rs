//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1324/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1324<F: Float>(t789: F, t9541: F, t2563: F, t2582: F, t2566: F, t786: F, t2578: F, t2570: F, t792: F, t118: F, t2379: F, t794: F) -> (F, F, F, F, F, F) {
    let t9542 = t9541 * t789;
    let t9544 = t2563 * t2582;
    let t9546 = t2566 * t786;
    let t9547 = t9546 * t2578;
    let t9549 = t792 * t2570;
    let t9551 = t118 * t794 * t2379;
    (t9542, t9544, t9546, t9547, t9549, t9551)
}
