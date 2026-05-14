//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 823/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk823<F: Float>(t1839: F, t546: F, t2003: F, t640: F, t19: F, t10: F, t6155: F, t1867: F, t28: F, t638: F, t667: F, t125: F, t1964: F, t1979: F, t2007: F, t554: F) -> (F, F, F, F, F, F, F, F) {
    let t6393 = t546 * t1839;
    let t6395 = t2003 * t640;
    let t6396 = t19 * t6395;
    let t6398 = t6155 * t10;
    let t6401 = 1.0 / t28 / t1867;
    let t6407 = t638 * t667;
    let t6421 = t1964 * t125;
    let t6427 = t554 * t2007 * t1979;
    (t6393, t6395, t6396, t6398, t6401, t6407, t6421, t6427)
}
