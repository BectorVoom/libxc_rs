//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1312/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1312(t51306: f64, t9500: f64, t54183: f64, t54186: f64, t54188: f64, t54190: f64, t54192: f64, t54194: f64, t54196: f64, t54199: f64, t54201: f64, t54203: f64, t54205: f64, t54207: f64) -> f64 {
    let t54209 = t51306 * t9500;
    let t54211 = t54183 / 96.0_f64 + t54186 / 48.0_f64 + t54188 / 24.0_f64 + t54190 / 96.0_f64 + t54192 / 128.0_f64 + t54194 / 128.0_f64 - t54196 / 32.0_f64 - t54199 + t54201 / 96.0_f64 - t54203 / 48.0_f64 - t54205 / 96.0_f64 - t54207 / 48.0_f64 + t54209 / 48.0_f64;
    t54211
}
