//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 798/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk798(t305: f64, t6072: f64, t296: f64, t413: f64, t2092: f64, t2096: f64, t2100: f64, t817: f64, t2106: f64, t814: f64, t816: f64, t322: f64) -> (f64, f64, f64, f64, f64) {
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = 0.47400060215270560269e0_f64 * t6075;
    let t6080 = t2092 * t2096;
    let t6086 = t2100 * t817;
    let t6089 = t814 * t2106;
    let t6094 = t816 * t816;
    let t6095 = 1.0_f64 / t6094;
    let t6096 = t322 * t6095;
    (t6076, t6080, t6086, t6089, t6096)
}
