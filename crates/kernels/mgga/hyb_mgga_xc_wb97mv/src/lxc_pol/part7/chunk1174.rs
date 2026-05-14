//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1174/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1174<F: Float>(t2025: F, t683: F, t8609: F, t1312: F, t2162: F, t8581: F, t8585: F, t2035: F, t6725: F, t8589: F, t8595: F, t8605: F, t3163: F, t6715: F, t25294: F, t3141: F) -> (F, F, F, F, F, F, F, F) {
    let t25660 = t683 * t2025 * t8609;
    let t25666 = t2162 * t1312;
    let t25672 = t683 * t2025 * t8581;
    let t25675 = t683 * t2025 * t8585;
    let t25681 = t2035 * t6725 * t8589;
    let t25684 = t683 * t8605 * t8595;
    let t25687 = t683 * t6715 * t3163;
    let t25689 = t25294 * t3141;
    (t25660, t25666, t25672, t25675, t25681, t25684, t25687, t25689)
}
