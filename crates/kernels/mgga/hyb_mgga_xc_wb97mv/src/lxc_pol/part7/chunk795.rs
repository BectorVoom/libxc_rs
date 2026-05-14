//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 795/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk795<F: Float>(t132: F, t1382: F, t1439: F, t338: F, t392: F, t4273: F, t4397: F, t322: F, t414: F, t422: F, t1631: F, t424: F, t4097: F, t428: F, t1476: F, t1484: F, t1455: F, t397: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t4401 = piecewise3(t134, 0.0, t4273 * t392 / 2.0 + t1382 * t1439 + t338 * t4397 / 2.0);
    let t4405 = t322 * t414;
    let t4406 = t4405 * t422;
    let t4407 = t424 * t1631;
    let t4408 = t428 * t4097;
    let t4409 = t4407 * t4408;
    let t4412 = t1476 * t1484;
    let t4415 = t397 * t1455;
    (t4401, t4406, t4409, t4412, t4415)
}
