//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 911/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk911<F: Float>(t1917: F, t1924: F, t1925: F, t6261: F, t3085: F, t81: F, t1205: F) -> (F, F, F, F, F) {
    let t8357 = t1924 * t1917;
    let t8360 = t6261 * t1925;
    let t8363 = t81 * t3085;
    let t8385 = t1924 * t1205;
    let t8386 = t81 * t1925;
    (t8357, t8360, t8363, t8385, t8386)
}
