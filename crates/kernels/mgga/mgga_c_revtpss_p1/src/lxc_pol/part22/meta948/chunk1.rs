//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3188/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3188<F: Float>(t1261: F, t12879: F, t247: F, t5056: F, t12963: F, t5323: F, t225: F, t56587: F, t17795: F, t3172: F, t3711: F, t17729: F, t17759: F, t44425: F) -> (F, F, F, F, F) {
    let t59233 = t1261 * t247 * t12879 * t5056;
    let t59239 = t5323 * t12963;
    let t59241 = t56587 * t225;
    let t59269 = t3711 * t3172 * t17795;
    let t59320 = t17729 * t44425 * t17759;
    (t59233, t59239, t59241, t59269, t59320)
}
