//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 716/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk716(t1195: f64, t840: f64, t1192: f64, t810: f64, t2376: f64, t2409: f64, t1193: f64, t892: f64, t338: f64, t938: f64, t3067: f64, t331: f64, t345: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4006 = 7.0_f64 / 288.0_f64 * t840 * t1195;
    let t4007 = t1192 * t810;
    let t4009 = t2409 * t2376 * t4007;
    let t4012 = t892 * t1193;
    let t4013 = t338 * t4012;
    let t4016 = t1192 * t938;
    let t4018 = t2409 * t3067 * t4016;
    let t4021 = t345 * t331;
    (t4006, t4007, t4009, t4013, t4016, t4018, t4021)
}
