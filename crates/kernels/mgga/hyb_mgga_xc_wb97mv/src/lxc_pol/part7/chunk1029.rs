//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1029/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1029<F: Float>(t3894: F, t54: F, t3877: F, t595: F, t57: F, t599: F, t60: F, t603: F, t63: F, t607: F, t66: F, t611: F, t3046: F, t3049: F, t3054: F, t3059: F, t3064: F, t3069: F, t3074: F, t583: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10392 = t54 * t3894;
    let t10395 = t595 * t3877;
    let t10400 = t57 * t3894;
    let t10403 = t599 * t3877;
    let t10408 = t60 * t3894;
    let t10411 = t603 * t3877;
    let t10416 = t63 * t3894;
    let t10419 = t607 * t3877;
    let t10424 = t66 * t3894;
    let t10427 = t611 * t3877;
    let t10432 = -t3049 * t3046 / 24.0 - t10392 * t583 / 48.0 - t10395 * t583 / 80.0 + t3054 * t3046 / 320.0 + t10400 * t583 / 640.0 + t10403 * t583 / 1152.0 - t3059 * t3046 / 5760.0 - t10408 * t583 / 11520.0 - t10411 * t583 / 21504.0 + t3064 * t3046 / 129024.0 + t10416 * t583 / 258048.0 + t10419 * t583 / 491520.0 - t3069 * t3046 / 3440640.0 - t10424 * t583 / 6881280.0 - t10427 * t583 / 13271040.0 + t3074 * t3046 / 0.10616832e9;
    (t10392, t10395, t10400, t10403, t10408, t10411, t10416, t10419, t10424, t10427, t10432)
}
