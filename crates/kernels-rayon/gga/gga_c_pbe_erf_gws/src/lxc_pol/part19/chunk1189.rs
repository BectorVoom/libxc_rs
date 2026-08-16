//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1189/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1189(t14229: f64, t14233: f64, t14551: f64, t14554: f64, t14558: f64, t14563: f64, t15249: f64, t15251: f64, t15253: f64, t15256: f64, t15258: f64, t15260: f64, t15262: f64, t15264: f64, t15266: f64, t15269: f64) -> f64 {
    let t15481 = -t15249 / 48.0_f64 - t15251 / 192.0_f64 + 5.0_f64 / 192.0_f64 * t15253 + t15256 / 24.0_f64 + t15258 / 8.0_f64 - 7.0_f64 / 288.0_f64 * t14551 - t15260 / 24.0_f64 + t15262 / 48.0_f64 + 7.0_f64 / 72.0_f64 * t14554 + 7.0_f64 / 144.0_f64 * t14558 + 7.0_f64 / 36.0_f64 * t14563 + t14229 + t14233 - t15264 / 96.0_f64 + t15266 / 192.0_f64 - t15269 / 48.0_f64;
    t15481
}
