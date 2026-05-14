//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1128/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1128<F: Float>(t3040: F, t703: F, t2077: F, t6134: F, t2068: F, t17: F, t2033: F, t696: F, t2064: F, t140: F, t21425: F, t35: F, t2073: F, t6623: F, t745: F, t177: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22055 = t3040 * t703;
    let t22057 = t6134 * t2077;
    let t22059 = t6134 * t2068;
    let t22068 = t17 / t696 / t2033;
    let t22069 = t2064 * t2064;
    let t22070 = 1.0 / t22069;
    let t22082 = 140.0 / 729.0 * t35 * t21425 * t140;
    let t22100 = t6134 * t2073;
    let t22191 = 1.0 / t6623 / t745;
    let t22200 = 1.0 / t6623 / t177;
    (t22055, t22057, t22059, t22068, t22070, t22082, t22100, t22191, t22200)
}
