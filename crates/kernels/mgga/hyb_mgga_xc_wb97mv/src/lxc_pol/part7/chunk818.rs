//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 818/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk818<F: Float>(t579: F, t6134: F, t1852: F, t1863: F, t1870: F, t1879: F, t572: F, t576: F, t17: F, t1858: F, t575: F) -> (F, F, F, F, F, F, F) {
    let t6135 = t6134 * t579;
    let t6137 = t1852 * t1863;
    let t6139 = t1852 * t1870;
    let t6141 = t1852 * t1879;
    let t6144 = 1.0 / t572 / t576;
    let t6145 = t17 * t6144;
    let t6147 = 1.0 / t1858 / t575;
    (t6135, t6137, t6139, t6141, t6144, t6145, t6147)
}
