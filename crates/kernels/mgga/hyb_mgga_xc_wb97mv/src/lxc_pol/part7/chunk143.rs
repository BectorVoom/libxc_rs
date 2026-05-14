//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 143/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk143<F: Float>(t212: F, t295: F, t398: F, t409: F, t412: F, t415: F, t417: F, t423: F, t424: F, t425: F, t428: F, t15: F, t221: F, t36: F) -> (F, F) {
    let t432 = t295 + 0.16e-2 * t398 * t409 + t412 * t212 + t415 * t417 + 0.8e-2 * t423 * t424 * t425 * t428;
    let t435 = t221 * t15 * t36;
    (t432, t435)
}
