//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2131/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2131<F: Float>(t1041: F, t248: F, t42749: F, t884: F, t1009: F, t10358: F, t1011: F, t1019: F, t10283: F, t969: F, t10189: F, t3014: F) -> (F, F, F, F, F) {
    let t42752 = t1041 * t248 * t42749 * t884;
    let t42754 = t10358 * t1009;
    let t42756 = t42754 * t1011 * t1019;
    let t42762 = t10283 * t969;
    let t42771 = t10189 * t3014;
    (t42752, t42754, t42756, t42762, t42771)
}
