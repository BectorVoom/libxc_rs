//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1188/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1188(t13895: f64, t14931: f64, t14962: f64, t14974: f64, t15187: f64, t15192: f64, t15195: f64, t15198: f64, t15201: f64, t15205: f64, t15209: f64, t15213: f64, t15216: f64, t15275: f64, t15279: f64, t15283: f64, t2408: f64, t3066: f64, t3207: f64, t335: f64, t3913: f64, t4002: f64) -> f64 {
    let t15285 = t15187 / 1536.0_f64 - t3913 * t4002 / 96.0_f64 - t15192 / 192.0_f64 + t3066 * t15195 / 24.0_f64 + t14931 + t15198 / 24.0_f64 + t15201 / 768.0_f64 - t15205 / 768.0_f64 - t3207 * t15209 / 16.0_f64 + t2408 * t15213 / 24.0_f64 + t15216 / 48.0_f64 + t13895 - t335 * t15275 / 96.0_f64 + t14962 + t15279 / 1536.0_f64 + t15283 / 384.0_f64 - t14974;
    t15285
}
