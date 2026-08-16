//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1247/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1247(t53272: f64, t3989: f64, t3990: f64, t3991: f64, t9080: f64, t345: f64, t6126: f64, t9297: f64, t1161: f64, t14106: f64, t2409: f64, t3066: f64, t3067: f64, t53234: f64, t53238: f64, t53243: f64, t53246: f64, t53248: f64, t53251: f64, t53253: f64, t53261: f64, t53264: f64, t53266: f64, t53270: f64, t9283: f64) -> f64 {
    let t53273 = 7.0_f64 / 144.0_f64 * t53272;
    let t53276 = t3989 * t3990 * t3991 * t9080;
    let t53283 = t345 * t6126;
    let t53286 = t3989 * t3990 * t53283 * t9297;
    let t53288 = -t53234 / 48.0_f64 + t53238 / 384.0_f64 - t53243 / 768.0_f64 - t53246 / 24.0_f64 - t53248 / 96.0_f64 - t53251 / 48.0_f64 + t3066 * t9283 * t53253 * t9297 / 4.0_f64 - t53261 + t53264 / 3072.0_f64 + t53266 / 48.0_f64 + t53270 / 512.0_f64 + t53273 - t53276 / 3072.0_f64 + t3066 * t2409 * t3067 * t14106 * t1161 / 48.0_f64 - t53286 / 512.0_f64;
    t53288
}
