//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 899/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk899<F: Float>(t25: F, t6401: F, t92: F, t1861: F, t2990: F, t1995: F, t2223: F, t125: F, t3: F, t544: F, t240: F) -> (F, F, F, F, F, F, F, F) {
    let t8141 = t25 * t6401;
    let t8142 = t8141 * t92;
    let t8143 = t2990 * t1861;
    let t8147 = t2223 * t1995;
    let t8148 = t8147 * t92;
    let t8149 = t125 * t3;
    let t8150 = t8149 * t544;
    let t8154 = t240 * t1995;
    let t8155 = t8154 * t92;
    (t8141, t8142, t8143, t8147, t8148, t8150, t8154, t8155)
}
