//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 816/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk816<F: Float>(t1298: F, t1519: F, t515: F, t509: F, t1616: F, t513: F, t17: F, t3038: F, sigma2: F) -> (F, F, F, F, F) {
    let t4955 = t1298 * sigma2;
    let t5395 = t515 * t1519;
    let t5427 = t509 * t1519;
    let t5722 = t513 * t1616;
    let t6129 = t3038 * t17;
    (t4955, t5395, t5427, t5722, t6129)
}
