//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1170/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1170<F: Float>(t21778: F, t25: F, t92: F, t2223: F, t6401: F, t638: F, t8141: F, t8147: F, t1995: F, t2987: F, t2991: F, t799: F, t8131: F, t8155: F, t6811: F, t8150: F) -> (F, F, F, F, F, F, F, F) {
    let t25446 = t25 * t21778 * t92;
    let t25452 = t2223 * t6401 * t92;
    let t25467 = t8141 * t638;
    let t25471 = t8147 * t638;
    let t25498 = t2987 * t799 * t1995 * t92 * t2991;
    let t25512 = t2987 * t8155 * t8131;
    let t25540 = t6811 * t1995 * t92;
    let t25542 = t2987 * t25540 * t8150;
    (t25446, t25452, t25467, t25471, t25498, t25512, t25540, t25542)
}
