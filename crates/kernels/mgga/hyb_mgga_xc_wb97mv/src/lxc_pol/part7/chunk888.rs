//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 888/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk888<F: Float>(t1142: F, t2839: F, t1153: F, t1157: F, t1126: F, t2822: F, sigma0: F) -> (F, F, F, F) {
    let t7849 = t1142 * t2839;
    let t7853 = t1153 * sigma0;
    let t7854 = t1157 * t7853;
    let t7897 = t1126 * t2822;
    (t7849, t7853, t7854, t7897)
}
