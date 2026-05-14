//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1225/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1225<F: Float>(t10224: F, t25556: F, t8140: F, t1224: F, t8147: F, t3864: F, t544: F, t667: F, t1044: F, t239: F, t24: F, t10231: F, t10273: F, t1173: F, t1978: F, t1993: F, t1995: F, t1996: F, t25430: F, t25467: F, t25471: F, t25551: F, t29705: F, t2987: F, t2989: F, t3: F, t3004: F, t3854: F, t453: F, t554: F, t557: F, t558: F, t6407: F, t6421: F, t6448: F, t8130: F, t8142: F, t8148: F, t8150: F, t8160: F, t8498: F, t8503: F) -> (F, F) {
    let t29733 = t8140 * t25556 * t10224;
    let t29735 = t8147 * t1224;
    let t29746 = t667 * t3864 * t544;
    let t29759 = t24 / t239 / t1044;
    let t29805 = 7.0 / 18.0 * t25551 * t8142 * t29705 - 7.0 / 216.0 * t29733 - t2987 * t29735 * t8150 / 6.0 - 7.0 / 72.0 * t8140 * t25467 * t10224 - t2987 * t25471 * t10231 / 6.0 + t8160 * t2989 * t29746 / 8.0 - 7.0 / 72.0 * t8140 * t8142 * t29746 - t2987 * t8148 * t8130 * t3 / 6.0 - t1993 * t29759 * t1995 * t558 * t453 / 6.0 - t1993 * t1996 * t6421 * t3864 / 48.0 - t1993 * t1996 * t6407 * t3864 / 24.0 - t1993 * t1996 * t6448 * t3864 / 48.0 - t554 * t557 * t25430 * t1173 / 32.0 - t554 * t3004 * t8503 * t3 / 8.0 - t554 * t3004 * t8498 * t3 / 8.0 - t554 * t557 * t6421 * t3854 / 64.0 - t554 * t557 * t6407 * t3854 / 32.0 - t554 * t557 * t1978 * t10273 / 32.0 - t554 * t557 * t6448 * t3854 / 64.0;
    (t29759, t29805)
}
