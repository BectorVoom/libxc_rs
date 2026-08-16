//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1025/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1025(t1161: f64, t2182: f64, t2376: f64, t2409: f64, t1105: f64, t2417: f64, t3067: f64, t1162: f64, t2220: f64, t338: f64, t1144: f64, t2402: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9213 = t1161 * t2182;
    let t9215 = t2409 * t2376 * t9213;
    let t9218 = t1105 * t2417;
    let t9220 = t2409 * t3067 * t9218;
    let t9224 = t338 * t2220 * t1162;
    let t9228 = t338 * t1144 * t2402;
    (t9213, t9215, t9218, t9220, t9224, t9228)
}
