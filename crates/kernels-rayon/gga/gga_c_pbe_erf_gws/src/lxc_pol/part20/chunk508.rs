//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 508/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk508(t43: f64, t318: f64, t1428: f64, t1098: f64, t19: f64, t796: f64, t801: f64, t1402: f64, t950: f64, t34: f64, t47: f64, t418: f64, t532: f64, param_gamma: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t2429 = param_gamma * t318;
    let t2449 = 4.0_f64 * t1428;
    let t2454 = t1098 * t796 * t19;
    let t2455 = t2454 * t801;
    let t2456 = 0.41076328840066666668e0_f64 * t2455;
    let t2457 = t1402 * t950;
    let t2460 = t47 * t34;
    let t2464 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t2457 * t418 + 8.0_f64 / 3.0_f64 * t2460 * t532);
    (t2429, t2449, t2454, t2455, t2456, t2457, t2464)
}
