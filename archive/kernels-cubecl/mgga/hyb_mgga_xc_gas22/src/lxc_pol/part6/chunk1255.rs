//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1255/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1255<F: Float>(t3785: F, t7810: F, t1166: F, t9593: F, t2828: F, t536: F, t7744: F, t3788: F, t9597: F, t1117: F, t9602: F, t2867: F, t9548: F) -> (F, F, F, F, F, F) {
    let t26194 = t3785 * t7810;
    let t26226 = t1166 * t9593;
    let t26231 = t536 * t2828 * t7744;
    let t26333 = t3788 * t9597;
    let t26345 = t1117 * t9602;
    let t26403 = t2867 * t9548;
    (t26194, t26226, t26231, t26333, t26345, t26403)
}
