//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1186/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1186(t14031: f64, t3765: f64, t3810: f64, t4039: f64, t11628: f64, t3139: f64, t4028: f64, t14073: f64, t14085: f64, t15070: f64, t15072: f64, t15074: f64, t15076: f64, t15249: f64, t15251: f64, t15253: f64, t15256: f64, t15258: f64, t15260: f64, t15262: f64) -> (f64, f64) {
    let t15264 = t14031 * t3765;
    let t15266 = t4039 * t3810;
    let t15268 = t3139 * t11628;
    let t15269 = t4028 * t15268;
    let t15271 = -t15249 / 96.0_f64 - t15251 / 384.0_f64 + 5.0_f64 / 384.0_f64 * t15253 + t15256 / 48.0_f64 + t15258 / 16.0_f64 - t15070 - t15260 / 48.0_f64 + t15262 / 96.0_f64 + t15072 + t15074 + t15076 + t14073 + t14085 - t15264 / 192.0_f64 + t15266 / 384.0_f64 - t15269 / 96.0_f64;
    (t15268, t15271)
}
