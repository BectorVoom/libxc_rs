//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 825/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk825<F: Float>(t554: F, t559: F, t6432: F, t1827: F, t92: F, t1988: F, t2007: F, t10: F, t1859: F, t1995: F, t549: F) -> (F, F, F, F, F) {
    let t6434 = t554 * t6432 * t559;
    let t6448 = t92 * t1827;
    let t6454 = t554 * t2007 * t1988;
    let t6456 = t1859 * t10;
    let t6461 = t549 * t1995;
    (t6434, t6448, t6454, t6456, t6461)
}
