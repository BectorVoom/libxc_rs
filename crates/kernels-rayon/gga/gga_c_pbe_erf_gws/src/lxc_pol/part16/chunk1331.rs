//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1331/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1331(t51201: f64, t51215: f64, t51222: f64, t54019: f64, t54021: f64, t54024: f64, t54027: f64, t54029: f64, t54031: f64, t54033: f64, t54035: f64, t54039: f64) -> f64 {
    let t55447 = -t54019 / 48.0_f64 - t54021 / 96.0_f64 - t54024 / 12.0_f64 + 119.0_f64 / 864.0_f64 * t51201 - t54027 / 12.0_f64 - t54029 / 12.0_f64 - t54031 / 96.0_f64 + 5.0_f64 / 48.0_f64 * t54033 - t54035 / 64.0_f64 + 7.0_f64 / 576.0_f64 * t51215 + 35.0_f64 / 108.0_f64 * t51222 + t54039 / 24.0_f64;
    t55447
}
