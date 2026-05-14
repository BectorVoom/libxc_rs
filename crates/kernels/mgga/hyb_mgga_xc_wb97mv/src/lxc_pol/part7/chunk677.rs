//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 677/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk677<F: Float>(t1364: F, t2300: F, t848: F, t3435: F, t838: F, t847: F, t2322: F) -> (F, F, F, F) {
    let t3452 = t2300 * t1364;
    let t3453 = t3452 * t848;
    let t3457 = t838 * t3435 * t847;
    let t3460 = t2322 * t1364;
    (t3452, t3453, t3457, t3460)
}
