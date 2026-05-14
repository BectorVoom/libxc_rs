//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 654/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk654<F: Float>(t2122: F, t739: F, t1264: F, t180: F, t2135: F) -> (F, F, F, F, F) {
    let t3248 = t2122 * t739;
    let t3249 = t180 * t1264;
    let t3252 = t1264 * t739;
    let t3262 = t2135 * t1264;
    let t3263 = t180 * t739;
    (t3248, t3249, t3252, t3262, t3263)
}
