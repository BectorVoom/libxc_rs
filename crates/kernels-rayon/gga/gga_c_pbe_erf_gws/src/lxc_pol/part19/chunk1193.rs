//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1193/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1193(t15543: f64, t2376: f64, t2409: f64, t12213: f64, t4216: f64, t14185: f64, t3742: f64, t9283: f64, t1115: f64, t14338: f64, t14918: f64, t15343: f64, t15346: f64, t15348: f64, t15358: f64, t15367: f64, t15372: f64, t15374: f64, t15378: f64, t15528: f64, t15532: f64, t15537: f64, t2408: f64, t3066: f64, t335: f64, t3913: f64, t4083: f64, t8629: f64) -> (f64, f64, f64, f64, f64) {
    let t15545 = t2409 * t2376 * t15543;
    let t15550 = t2409 * t12213 * t4216;
    let t15558 = t14185 * t3742;
    let t15559 = t9283 * t15558;
    let t15565 = t14338 + t3066 * t15528 / 48.0_f64 - t335 * t15532 / 48.0_f64 + t8629 * t15537 / 96.0_f64 - t15343 / 48.0_f64 - t15346 / 24.0_f64 - t15348 / 12.0_f64 + t2408 * t15545 / 48.0_f64 - t15358 / 1536.0_f64 + t3066 * t15550 / 24.0_f64 - t15367 / 1536.0_f64 - t1115 * t14918 / 48.0_f64 - t3913 * t4083 / 96.0_f64 - t2408 * t15559 / 12.0_f64 + t15372 / 768.0_f64 + t15374 / 48.0_f64 + t15378 / 48.0_f64;
    (t15545, t15550, t15558, t15559, t15565)
}
