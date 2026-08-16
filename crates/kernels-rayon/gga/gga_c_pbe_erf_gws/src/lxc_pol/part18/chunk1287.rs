//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1287/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1287(t13984: f64, t56320: f64, t13972: f64, t15371: f64, t1105: f64, t14576: f64, t2376: f64, t2408: f64, t2409: f64, t53273: f64, t53302: f64, t53308: f64, t55074: f64, t56299: f64, t56302: f64, t56305: f64, t56307: f64, t56309: f64, t56312: f64, t56316: f64, t56318: f64) -> f64 {
    let t56321 = t56320 * t13984;
    let t56323 = t13972 * t15371;
    let t56330 = t56299 / 512.0_f64 + t56302 / 1536.0_f64 + t56305 / 384.0_f64 - t56307 / 48.0_f64 - t56309 / 24.0_f64 - 5.0_f64 / 384.0_f64 * t56312 - t56316 / 96.0_f64 - t56318 / 24.0_f64 - t56321 / 96.0_f64 + t53273 - t53302 - t53308 - t55074 - 7.0_f64 / 2304.0_f64 * t56323 + t2408 * t2409 * t2376 * t14576 * t1105 / 24.0_f64;
    t56330
}
