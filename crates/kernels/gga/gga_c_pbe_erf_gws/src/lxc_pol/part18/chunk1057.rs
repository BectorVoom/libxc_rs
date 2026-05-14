//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1057/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1057<F: Float>(t2409: F, t9897: F, t3965: F, t9818: F, t14121: F, t1105: F, t4182: F, t2376: F, t1192: F, t3717: F, t1115: F, t13948: F, t14437: F, t14718: F, t14978: F, t14986: F, t14996: F, t15289: F, t15292: F, t15297: F, t15300: F, t15310: F, t15312: F, t2408: F, t335: F, t3921: F, t4002: F) -> (F, F, F, F, F, F, F) {
    let t15314 = t2409 * t9897;
    let t15315 = t3965 * t15314;
    let t15317 = t2409 * t9818;
    let t15318 = t14121 * t15317;
    let t15320 = t4182 * t1105;
    let t15322 = t2409 * t2376 * t15320;
    let t15325 = t1192 * t3717;
    let t15327 = t2409 * t2376 * t15325;
    let t15330 = t15289 / 96.0 - t335 * t15292 / 48.0 + t15297 / 1536.0 - t335 * t15300 / 96.0 - t13948 - t3921 * t4002 / 96.0 - t14978 - t14986 - t1115 * t14437 / 48.0 + 7.0 / 144.0 * t14718 + t14996 + 5.0 / 768.0 * t15310 - t15312 / 48.0 - t15315 / 96.0 + t15318 / 16.0 + t2408 * t15322 / 24.0 + t2408 * t15327 / 48.0;
    (t15314, t15317, t15320, t15322, t15325, t15327, t15330)
}
