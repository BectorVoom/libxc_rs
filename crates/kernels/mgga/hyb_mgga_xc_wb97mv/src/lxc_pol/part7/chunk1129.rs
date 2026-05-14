//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1129/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1129<F: Float>(t2120: F, t6623: F, t180: F, t2122: F, t6491: F, t676: F, t136: F, t3003: F, t764: F, t2015: F, t2022: F, t6745: F, t215: F, t8473: F, t6965: F, t834: F) -> (F, F, F, F, F, F, F, F) {
    let t22208 = 1.0 / t6623 / t2120;
    let t22271 = t180 * t2122;
    let t22292 = t676 * t6491;
    let t22311 = t136 * t3003 * t764;
    let t22314 = t2015 * t2022;
    let t22316 = t676 * t6745;
    let t22320 = 5.0 / 108.0 * t136 * t8473 * t215;
    let t22394 = t834 * t6965;
    (t22208, t22271, t22292, t22311, t22314, t22316, t22320, t22394)
}
