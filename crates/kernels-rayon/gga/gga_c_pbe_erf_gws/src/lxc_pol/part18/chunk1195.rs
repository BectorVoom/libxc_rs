//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1195/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1195(t3781: f64, t3950: f64, t850: f64, t833: f64, t13989: f64, t14770: f64, t14779: f64, t14999: f64, t15332: f64, t15335: f64, t15338: f64, t15343: f64, t15346: f64, t15348: f64, t15353: f64, t15358: f64, t15362: f64, t15367: f64, t15372: f64, t15374: f64, t2408: f64, t3066: f64) -> (f64, f64) {
    let t15377 = t850 * t3781 * t3950;
    let t15378 = t15377 * t833;
    let t15380 = t14999 - t15332 / 24.0_f64 - t15335 / 48.0_f64 + t13989 - t2408 * t15338 / 12.0_f64 - t15343 / 96.0_f64 - t15346 / 48.0_f64 - t15348 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t14770 - t3066 * t15353 / 16.0_f64 - t15358 / 3072.0_f64 + t3066 * t15362 / 24.0_f64 - t15367 / 3072.0_f64 + 7.0_f64 / 144.0_f64 * t14779 + t15372 / 1536.0_f64 + t15374 / 96.0_f64 + t15378 / 96.0_f64;
    (t15377, t15380)
}
