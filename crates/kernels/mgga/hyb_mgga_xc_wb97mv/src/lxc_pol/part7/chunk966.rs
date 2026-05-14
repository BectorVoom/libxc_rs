//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 966/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk966<F: Float>(t3487: F, t929: F, t238: F, t242: F, t341: F, t9290: F, t1399: F, t2224: F) -> (F, F, F, F, F) {
    let t9351 = t929 * t3487;
    let t9353 = t238 * t242 * t9351;
    let t9355 = t341 * t9290;
    let t9357 = t238 * t242 * t9355;
    let t9360 = t238 * t2224 * t1399;
    (t9351, t9353, t9355, t9357, t9360)
}
