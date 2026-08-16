//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 707/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk707(t353: f64, t4111: f64, t338: f64, t2408: f64, t3066: f64, t335: f64, t3953: f64, t3961: f64, t3963: f64, t3967: f64, t3977: f64, t3986: f64, t3994: f64, t3998: f64, t4072: f64, t4077: f64, t4083: f64, t4087: f64, t4090: f64, t4094: f64, t4099: f64, t827: f64) -> (f64, f64) {
    let t4112 = t353 * t4111;
    let t4113 = t338 * t4112;
    let t4116 = t3953 / 48.0_f64 - t4072 - t3961 / 24.0_f64 + t3963 / 48.0_f64 - t3967 / 48.0_f64 + t3977 / 768.0_f64 - t4077 - t3986 / 384.0_f64 - t3994 / 1536.0_f64 - t3998 / 1536.0_f64 - t827 * t4083 / 96.0_f64 + t4087 + t2408 * t4090 / 48.0_f64 - t335 * t4094 / 96.0_f64 + t3066 * t4099 / 48.0_f64 - t335 * t4113 / 96.0_f64;
    (t4113, t4116)
}
