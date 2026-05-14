//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 640/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk640<F: Float>(t615: F, t82: F, t79: F, t1205: F, t3085: F, t3090: F, t3103: F, t3104: F, t3110: F, t622: F, t626: F, t74: F, t81: F, t1217: F, t1929: F, t3089: F, t3093: F, t617: F, t631: F, t72: F, t85: F) -> (F, F, F, F) {
    let t3113 = t615 * t82;
    let t3116 = t79 * t615;
    let t3122 = -2.0 * t3103 * t3104 + t622 * t3085 * t81 / 2.0 + t3110 * t3104 / 4.0 - 4.0 * t3113 * t1205 - t3116 * t3090 - 4.0 * t626 * t3085 - t74 * t3085 * t81;
    let t3125 = -t3089 * t3090 / 2.0 + 2.0 * t1929 * t3093 - t617 * t3085 + 2.0 * t3085 * t85 + 2.0 * t1205 * t631 + 2.0 * t615 * t1217 + 2.0 * t72 * t3122;
    (t3113, t3116, t3122, t3125)
}
