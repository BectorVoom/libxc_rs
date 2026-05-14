//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1098/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1098<F: Float>(t13780: F, t13859: F, t3990: F, t8764: F, t14733: F, t4390: F, t13979: F, t14437: F, t14757: F, t2384: F, t2392: F, t2408: F, t2409: F, t3066: F, t36129: F, t4016: F, t51096: F, t51102: F, t53351: F, t53354: F, t53355: F, t53357: F, t53362: F, t53374: F, t6781: F, t8589: F) -> (F,) {
    let t53378 = t13859 * t3990 * t13780 * t8764;
    let t53386 = t14733 * t4390;
    let t53390 = -t53351 / 1536.0 + t53354 + t53355 / 24.0 + t53357 / 96.0 + t53362 / 768.0 - 7.0 / 2304.0 * t51096 + t2408 * t2409 * t8589 * t13979 / 48.0 + t2408 * t2409 * t6781 * t14757 / 24.0 - t53374 - 7.0 / 72.0 * t51102 + t53378 / 768.0 + t3066 * t2409 * t36129 * t4016 / 24.0 - t2392 * t14437 / 96.0 + t53386 / 24.0 - t2384 * t14437 / 96.0;
    (t53390,)
}
