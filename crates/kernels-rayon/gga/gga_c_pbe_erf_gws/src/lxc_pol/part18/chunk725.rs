//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 725/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk725(t353: f64, t4053: f64, t338: f64, t2408: f64, t3066: f64, t335: f64, t3953: f64, t3957: f64, t3961: f64, t3963: f64, t3967: f64, t3977: f64, t3981: f64, t3986: f64, t3994: f64, t3998: f64, t4002: f64, t4006: f64, t4009: f64, t4013: f64, t4018: f64, t827: f64) -> (f64, f64) {
    let t4054 = t353 * t4053;
    let t4055 = t338 * t4054;
    let t4058 = t3953 / 96.0_f64 - t3957 - t3961 / 48.0_f64 + t3963 / 96.0_f64 - t3967 / 96.0_f64 + t3977 / 1536.0_f64 - t3981 - t3986 / 768.0_f64 - t3994 / 3072.0_f64 - t3998 / 3072.0_f64 - t827 * t4002 / 96.0_f64 + t4006 + t2408 * t4009 / 48.0_f64 - t335 * t4013 / 96.0_f64 + t3066 * t4018 / 48.0_f64 - t335 * t4055 / 96.0_f64;
    (t4055, t4058)
}
