//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1156/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1156<F: Float>(t23749: F, t23807: F, t23871: F, t23955: F, t458: F, t464: F, t2708: F, t453: F, t492: F, t1046: F, t7697: F, t456: F, t454: F, t7543: F, t1099: F, t2765: F, t7768: F) -> (F, F, F, F, F, F) {
    let t23959 = t458 * t464 * (t23749 + t23807 + t23871 + t23955);
    let t23964 = t453 * t2708 * t492;
    let t23970 = 16.0 * t1046 * t7697;
    let t23971 = t456 * t456;
    let t23975 = 840.0 * t454 / t23971 * t492;
    let t23976 = t1046 * t7543;
    let t23980 = 0.21053605041484726346e2 * t1099 * t7768 * t2765;
    (t23959, t23964, t23970, t23975, t23976, t23980)
}
