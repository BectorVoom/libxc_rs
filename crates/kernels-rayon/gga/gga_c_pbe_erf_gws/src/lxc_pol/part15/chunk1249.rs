//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1249/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1249(t4146: f64, t51818: f64, t14797: f64, t3989: f64, t3990: f64, t9321: f64, t13781: f64, t14582: f64, t3972: f64, t9380: f64, t12213: f64, t13840: f64, t14446: f64, t2352: f64, t2376: f64, t2408: f64, t2409: f64, t2494: f64, t26654: f64, t27112: f64, t3066: f64, t3067: f64, t3306: f64, t4007: f64, t4052: f64, t4164: f64, t4182: f64, t53299: f64, t53302: f64, t53308: f64, t53323: f64, t53327: f64, t6781: f64) -> f64 {
    let t53334 = t51818 * t4146;
    let t53338 = t3989 * t3990 * t14797 * t9321;
    let t53346 = t3972 * t13781 * t14582 * t9380;
    let t53348 = t2408 * t2409 * t2376 * t4052 * t2494 / 24.0_f64 + t53299 / 768.0_f64 - t53302 + t3066 * t2409 * t27112 * t4164 / 48.0_f64 - t53308 + t2408 * t2409 * t6781 * t14446 / 24.0_f64 + t2408 * t2409 * t26654 * t4007 / 24.0_f64 + t3066 * t2409 * t3067 * t4182 * t2352 / 48.0_f64 - t53323 / 768.0_f64 - t53327 / 384.0_f64 + t3066 * t2409 * t3067 * t4052 * t3306 / 24.0_f64 - 119.0_f64 / 13824.0_f64 * t53334 + t53338 / 1536.0_f64 + t3066 * t2409 * t12213 * t13840 / 48.0_f64 - t53346 / 1536.0_f64;
    t53348
}
