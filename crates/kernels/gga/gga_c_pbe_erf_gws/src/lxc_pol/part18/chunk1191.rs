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
    let t15330 = t15289 / F::new(96.0) - t335 * t15292 / F::new(48.0) + t15297 / F::new(1536.0) - t335 * t15300 / F::new(96.0) - t13948 - t3921 * t4002 / F::new(96.0) - t14978 - t14986 - t1115 * t14437 / F::new(48.0) + F::new(7.0) / F::new(144.0) * t14718 + t14996 + F::new(5.0) / F::new(768.0) * t15310 - t15312 / F::new(48.0) - t15315 / F::new(96.0) + t15318 / F::new(16.0) + t2408 * t15322 / F::new(24.0) + t2408 * t15327 / F::new(48.0);
    (t15322, t15325, t15327, t15330)
}
