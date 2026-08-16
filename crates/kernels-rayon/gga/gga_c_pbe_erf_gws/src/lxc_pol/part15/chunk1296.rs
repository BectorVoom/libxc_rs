//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1296/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1296(t51421: f64, t9512: f64, t14007: f64, t9570: f64, t51222: f64, t4023: f64, t9179: f64, t51215: f64, t54019: f64, t54021: f64, t54024: f64, t54026: f64, t54027: f64, t54029: f64, t54031: f64) -> f64 {
    let t54033 = t51421 * t9512;
    let t54035 = t14007 * t9570;
    let t54038 = 35.0_f64 / 216.0_f64 * t51222;
    let t54039 = t9179 * t4023;
    let t54041 = -t54019 / 96.0_f64 - t54021 / 192.0_f64 - t54024 / 24.0_f64 + t54026 - t54027 / 24.0_f64 - t54029 / 24.0_f64 - t54031 / 192.0_f64 + 5.0_f64 / 96.0_f64 * t54033 - t54035 / 128.0_f64 + 7.0_f64 / 1152.0_f64 * t51215 + t54038 + t54039 / 48.0_f64;
    t54041
}
