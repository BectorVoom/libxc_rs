//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1114/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1114(t2367: f64, t4083: f64, t1205: f64, t2074: f64, t2376: f64, t2409: f64, t14004: f64, t14008: f64, t14012: f64, t14016: f64, t14018: f64, t14020: f64, t14025: f64, t14029: f64, t14032: f64, t14036: f64, t14038: f64, t14040: f64, t14042: f64, t14047: f64, t14050: f64, t14052: f64) -> (f64, f64, f64, f64) {
    let t14198 = t2367 * t4083;
    let t14200 = t1205 * t2074;
    let t14202 = t2409 * t2376 * t14200;
    let t14222 = t14004 / 48.0_f64 - t14008 / 384.0_f64 + t14012 / 48.0_f64 - t14016 / 48.0_f64 + t14018 / 48.0_f64 + t14020 / 48.0_f64 - 7.0_f64 / 72.0_f64 * t14025 - 7.0_f64 / 288.0_f64 * t14029 - t14032 / 96.0_f64 + t14036 / 128.0_f64 + t14038 / 12.0_f64 - t14040 / 24.0_f64 + 7.0_f64 / 36.0_f64 * t14042 + 7.0_f64 / 72.0_f64 * t14047 - t14050 / 48.0_f64 + t14052 / 8.0_f64;
    (t14198, t14200, t14202, t14222)
}
