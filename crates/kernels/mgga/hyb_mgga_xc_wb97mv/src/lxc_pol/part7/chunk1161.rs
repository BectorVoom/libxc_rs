//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1161/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1161<F: Float>(t1795: F, t522: F, t2822: F, t2840: F, t2952: F, t7853: F, t1122: F, t297: F, t1126: F, t10064: F, t1153: F, t1157: F, t521: F, t8019: F, t2848: F, t10075: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24192 = t1795 * t522;
    let t24218 = t2840 * t2822;
    let t24237 = t2952 * t7853;
    let t24244 = t1122 * t297;
    let t24245 = t1126 * t24244;
    let t24260 = t1126 * t10064;
    let t24272 = t1153 * t297;
    let t24273 = t1157 * t24272;
    let t24434 = 1.0 / t8019 / t521;
    let t24504 = t2952 * t2848;
    let t24605 = t1157 * t10075;
    (t24192, t24218, t24237, t24244, t24245, t24260, t24272, t24273, t24434, t24504, t24605)
}
