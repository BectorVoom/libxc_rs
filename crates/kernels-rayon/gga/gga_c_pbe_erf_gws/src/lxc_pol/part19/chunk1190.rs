//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1190/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1190(t15466: f64, t15481: f64, t898: f64, t338: f64, t353: f64, t1205: f64, t3703: f64, t2376: f64, t2409: f64, t4207: f64, t8589: f64, t1115: f64, t14295: f64, t14302: f64, t14611: f64, t14655: f64, t14911: f64, t14964: f64, t15192: f64, t15198: f64, t15201: f64, t15205: f64, t15216: f64, t15279: f64, t15445: f64, t2408: f64, t3066: f64, t3207: f64, t335: f64, t3921: f64, t4083: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15482 = t15466 + t15481;
    let t15483 = t898 * t15482;
    let t15485 = t338 * t353 * t15483;
    let t15490 = t1205 * t3703;
    let t15492 = t2409 * t2376 * t15490;
    let t15500 = t2409 * t8589 * t4207;
    let t15503 = -t3921 * t4083 / 96.0_f64 - t1115 * t14911 / 48.0_f64 - t15192 / 96.0_f64 + t3066 * t15445 / 24.0_f64 + 7.0_f64 / 1152.0_f64 * t14611 + t15198 / 12.0_f64 - t335 * t15485 / 96.0_f64 + t15201 / 384.0_f64 - t15205 / 384.0_f64 - t3207 * t15492 / 16.0_f64 + t15216 / 24.0_f64 + t14295 + 7.0_f64 / 288.0_f64 * t14655 - t14302 + t15279 / 768.0_f64 - 7.0_f64 / 72.0_f64 * t14964 + t2408 * t15500 / 24.0_f64;
    (t15482, t15483, t15485, t15490, t15492, t15500, t15503)
}
