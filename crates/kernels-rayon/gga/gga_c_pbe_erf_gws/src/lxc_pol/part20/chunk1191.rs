//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1191/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1191(t15320: f64, t2376: f64, t2409: f64, t1192: f64, t3717: f64, t1115: f64, t13948: f64, t14437: f64, t14718: f64, t14978: f64, t14986: f64, t14996: f64, t15289: f64, t15292: f64, t15297: f64, t15300: f64, t15310: f64, t15312: f64, t15315: f64, t15318: f64, t2408: f64, t335: f64, t3921: f64, t4002: f64) -> (f64, f64, f64, f64) {
    let t15322 = t2409 * t2376 * t15320;
    let t15325 = t1192 * t3717;
    let t15327 = t2409 * t2376 * t15325;
    let t15330 = t15289 / 96.0_f64 - t335 * t15292 / 48.0_f64 + t15297 / 1536.0_f64 - t335 * t15300 / 96.0_f64 - t13948 - t3921 * t4002 / 96.0_f64 - t14978 - t14986 - t1115 * t14437 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t14718 + t14996 + 5.0_f64 / 768.0_f64 * t15310 - t15312 / 48.0_f64 - t15315 / 96.0_f64 + t15318 / 16.0_f64 + t2408 * t15322 / 24.0_f64 + t2408 * t15327 / 48.0_f64;
    (t15322, t15325, t15327, t15330)
}
