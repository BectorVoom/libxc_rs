//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 867/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk867<F: Float>(t1078: F, t1085: F, t7523: F, t1099: F, t2694: F, t2697: F, t1077: F, t2688: F) -> (F, F, F, F) {
    let t7525 = t1078 * t7523 * t1085;
    let t7527 = 0.5848223622634646207e0 * t1099 * t7525;
    let t7528 = t2697 * t2694;
    let t7531 = 1.0 / t2688 / t1077;
    (t7525, t7527, t7528, t7531)
}
