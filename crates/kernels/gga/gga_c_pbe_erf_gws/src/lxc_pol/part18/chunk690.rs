//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 690/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk690<F: Float>(t353: F, t4053: F, t338: F, t2408: F, t3066: F, t335: F, t3953: F, t3957: F, t3961: F, t3963: F, t3967: F, t3977: F, t3981: F, t3986: F, t3994: F, t3998: F, t4002: F, t4006: F, t4009: F, t4013: F, t4018: F, t827: F) -> (F, F) {
    let t4054 = t353 * t4053;
    let t4055 = t338 * t4054;
    let t4058 = t3953 / 96.0 - t3957 - t3961 / 48.0 + t3963 / 96.0 - t3967 / 96.0 + t3977 / 1536.0 - t3981 - t3986 / 768.0 - t3994 / 3072.0 - t3998 / 3072.0 - t827 * t4002 / 96.0 + t4006 + t2408 * t4009 / 48.0 - t335 * t4013 / 96.0 + t3066 * t4018 / 48.0 - t335 * t4055 / 96.0;
    (t4055, t4058)
}
