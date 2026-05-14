//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 797/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk797<F: Float>(t399: F, t400: F, t196: F, t4081: F, t195: F, t1457: F, t396: F, t397: F, t296: F, t194: F, t406: F, t407: F, t313: F, t4097: F, t316: F, t408: F) -> (F, F, F, F, F, F, F, F) {
    let t4445 = t400 * t399;
    let t4447 = 1.0 / t196 / t4445;
    let t4452 = t400 * t4081;
    let t4454 = 1.0 / t195 / t4452;
    let t4459 = t4454 * t1457;
    let t4462 = t397 * t396;
    let t4463 = t296 * t4462;
    let t4464 = t400 * t400;
    let t4465 = t4464 * t194;
    let t4466 = 1.0 / t4465;
    let t4468 = 1.0 / t407 / t406;
    let t4469 = t4466 * t4468;
    let t4472 = t313 * t4097;
    let t4475 = t316 * t4097;
    let t4478 = t4447 * t408;
    (t4447, t4454, t4459, t4463, t4469, t4472, t4475, t4478)
}
