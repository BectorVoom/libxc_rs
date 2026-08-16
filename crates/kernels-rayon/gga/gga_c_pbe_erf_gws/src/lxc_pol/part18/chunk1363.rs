//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1363/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1363(t15292: f64, t840: f64, t361: f64, t57321: f64, t13917: f64, t3223: f64, t12014: f64, t13919: f64, t1115: f64, t13911: f64, t13930: f64, t14397: f64, t15273: f64, t2498: f64, t335: f64, t338: f64, t35057: f64, t4002: f64, t53617: f64, t53939: f64, t54488: f64, t57402: f64, t57404: f64, t57410: f64, t57415: f64, t57422: f64, t8629: f64, t8793: f64, t892: f64, t9858: f64) -> f64 {
    let t57428 = t840 * t15292;
    let t57432 = t361 * t57321;
    let t57434 = t13917 * t57432 * t3223;
    let t57441 = t13917 * t13919 * t12014;
    let t57445 = t57402 / 24.0_f64 + t57404 / 24.0_f64 - t2498 * t14397 / 48.0_f64 - t57410 / 192.0_f64 - t1115 * t54488 / 48.0_f64 - t57415 / 192.0_f64 - t9858 * t4002 / 96.0_f64 - t57422 / 1536.0_f64 - t335 * t338 * t892 * t15273 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t57428 + t35057 * t13911 / 48.0_f64 - t57434 / 1536.0_f64 + t8629 * t53939 / 48.0_f64 + t35057 * t13930 / 48.0_f64 - t57441 / 1536.0_f64 + t8793 * t53617 / 24.0_f64;
    t57445
}
