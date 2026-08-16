//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1251/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1251(t14749: f64, t9270: f64, t13780: f64, t13859: f64, t3990: f64, t8764: f64, t14733: f64, t4390: f64, t13979: f64, t14437: f64, t14757: f64, t2384: f64, t2392: f64, t2408: f64, t2409: f64, t3066: f64, t36129: f64, t4016: f64, t51096: f64, t51102: f64, t53351: f64, t53354: f64, t53355: f64, t53357: f64, t53362: f64, t6781: f64, t8589: f64) -> f64 {
    let t53374 = 7.0_f64 / 72.0_f64 * t9270 * t14749;
    let t53378 = t13859 * t3990 * t13780 * t8764;
    let t53386 = t14733 * t4390;
    let t53390 = -t53351 / 1536.0_f64 + t53354 + t53355 / 24.0_f64 + t53357 / 96.0_f64 + t53362 / 768.0_f64 - 7.0_f64 / 2304.0_f64 * t51096 + t2408 * t2409 * t8589 * t13979 / 48.0_f64 + t2408 * t2409 * t6781 * t14757 / 24.0_f64 - t53374 - 7.0_f64 / 72.0_f64 * t51102 + t53378 / 768.0_f64 + t3066 * t2409 * t36129 * t4016 / 24.0_f64 - t2392 * t14437 / 96.0_f64 + t53386 / 24.0_f64 - t2384 * t14437 / 96.0_f64;
    t53390
}
