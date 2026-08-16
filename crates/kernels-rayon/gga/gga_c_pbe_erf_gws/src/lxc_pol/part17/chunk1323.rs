//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1323/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1323(t14023: f64, t14548: f64, t863: f64, t51412: f64, t14547: f64, t28029: f64, t6523: f64, t14031: f64, t9556: f64, t51415: f64, t54315: f64, t54317: f64, t54320: f64, t54321: f64, t54323: f64, t54324: f64, t54326: f64) -> f64 {
    let t54329 = t863 * t14023 * t14548;
    let t54330 = 7.0_f64 / 24.0_f64 * t54329;
    let t54331 = 35.0_f64 / 108.0_f64 * t51412;
    let t54333 = t14547 * t6523 * t28029;
    let t54335 = t14031 * t9556;
    let t54337 = t54315 / 24.0_f64 + t54317 / 24.0_f64 - t54320 - t54321 - t54323 - t54324 / 96.0_f64 - t54326 / 192.0_f64 - t54330 - t54331 - t51415 + t54333 / 16.0_f64 - t54335 / 384.0_f64;
    t54337
}
