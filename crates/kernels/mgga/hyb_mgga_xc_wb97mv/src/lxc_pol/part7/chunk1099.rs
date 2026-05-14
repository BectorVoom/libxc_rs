//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1099/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1099<F: Float>(t11618: F, t11627: F, t11633: F, t11646: F, t540: F, t1169: F, t4519: F, t1558: F, t3673: F, t1512: F, t3842: F, t1104: F, t4642: F, t4529: F, t7917: F, t7907: F) -> (F, F, F, F, F, F, F, F) {
    let t11648 = t11618 + t11627 + t11633 + t11646;
    let t11649 = t11648 * t540;
    let t11650 = t4519 * t1169;
    let t11651 = t3673 * t1558;
    let t11653 = t1512 * t3842;
    let t11655 = t1104 * t4642;
    let t11659 = t7917 * t4529;
    let t11668 = t7907 * t4529;
    (t11648, t11649, t11650, t11651, t11653, t11655, t11659, t11668)
}
