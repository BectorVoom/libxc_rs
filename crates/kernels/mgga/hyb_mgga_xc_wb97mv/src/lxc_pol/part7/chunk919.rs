//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 919/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk919<F: Float>(t1173: F, t6421: F, t6407: F, t1178: F, t554: F, t6432: F, t2007: F, t3015: F, t125: F, t3129: F, t544: F, t1224: F, t667: F, t1877: F, t3014: F, t557: F, t6427: F, t6430: F, t6434: F, t6454: F, t6463: F, t6466: F, t6479: F, t6482: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8484 = t6421 * t1173;
    let t8488 = t6407 * t1173;
    let t8493 = t554 * t6432 * t1178;
    let t8497 = t554 * t2007 * t3015 / 96.0;
    let t8498 = t3129 * t125;
    let t8499 = t8498 * t544;
    let t8503 = t1224 * t667;
    let t8504 = t8503 * t544;
    let t8508 = t3014 * t1877;
    let t8515 = -t6427 / 96.0 - t6430 / 96.0 + t6434 / 144.0 - t6454 / 192.0 - t6463 / 144.0 - t554 * t557 * t8484 / 64.0 - t554 * t557 * t8488 / 32.0 + t8493 / 288.0 - t8497 - t554 * t557 * t8499 / 32.0 - t554 * t557 * t8504 / 32.0 - t554 * t557 * t8508 / 64.0 - t6466 / 64.0 - t6479 / 64.0 - t6482 / 32.0;
    (t8484, t8488, t8493, t8497, t8498, t8499, t8503, t8504, t8508, t8515)
}
