//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1191/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1191<F: Float>(t15320: F, t2376: F, t2409: F, t1192: F, t3717: F, t1115: F, t13948: F, t14437: F, t14718: F, t14978: F, t14986: F, t14996: F, t15289: F, t15292: F, t15297: F, t15300: F, t15310: F, t15312: F, t15315: F, t15318: F, t2408: F, t335: F, t3921: F, t4002: F) -> (F, F, F, F) {
    let t15322 = t2409 * t2376 * t15320;
    let t15325 = t1192 * t3717;
    let t15327 = t2409 * t2376 * t15325;
    let t15330 = t15289 / F::cast_from(96.0_f64) - t335 * t15292 / F::cast_from(48.0_f64) + t15297 / F::cast_from(1536.0_f64) - t335 * t15300 / F::cast_from(96.0_f64) - t13948 - t3921 * t4002 / F::cast_from(96.0_f64) - t14978 - t14986 - t1115 * t14437 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14718 + t14996 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t15310 - t15312 / F::cast_from(48.0_f64) - t15315 / F::cast_from(96.0_f64) + t15318 / F::cast_from(16.0_f64) + t2408 * t15322 / F::cast_from(24.0_f64) + t2408 * t15327 / F::cast_from(48.0_f64);
    (t15322, t15325, t15327, t15330)
}
