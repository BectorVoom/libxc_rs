//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1187/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1187<F: Float>(t2283: F, t3380: F, t1345: F, t6902: F, t2516: F, t3491: F, t1390: F, t7402: F, t9294: F, t937: F, t1386: F, t222: F, t6129: F) -> (F, F, F, F, F, F) {
    let t26929 = t3380 * t2283;
    let t26934 = t1345 * t6902;
    let t26998 = t3491 * t2516;
    let t27001 = t1390 * t7402;
    let t27010 = t9294 * t937;
    let t27021 = t222 * t6129 * t1386;
    (t26929, t26934, t26998, t27001, t27010, t27021)
}
