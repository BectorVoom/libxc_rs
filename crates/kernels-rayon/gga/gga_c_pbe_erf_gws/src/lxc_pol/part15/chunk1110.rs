//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1110/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1110(t14046: f64, t4029: f64, t3139: f64, t6178: f64, t4028: f64, t1184: f64, t2212: f64, t14004: f64, t14008: f64, t14012: f64, t14016: f64, t14018: f64, t14020: f64, t14026: f64, t14030: f64, t14032: f64, t14036: f64, t14038: f64, t14040: f64, t14043: f64) -> (f64, f64, f64) {
    let t14047 = t14046 * t4029;
    let t14048 = 7.0_f64 / 144.0_f64 * t14047;
    let t14049 = t3139 * t6178;
    let t14050 = t4028 * t14049;
    let t14052 = t1184 * t2212;
    let t14054 = t14004 / 96.0_f64 - t14008 / 768.0_f64 + t14012 / 96.0_f64 - t14016 / 96.0_f64 + t14018 / 96.0_f64 + t14020 / 96.0_f64 - t14026 - t14030 - t14032 / 192.0_f64 + t14036 / 256.0_f64 + t14038 / 24.0_f64 - t14040 / 48.0_f64 + t14043 + t14048 - t14050 / 96.0_f64 + t14052 / 16.0_f64;
    (t14047, t14049, t14054)
}
