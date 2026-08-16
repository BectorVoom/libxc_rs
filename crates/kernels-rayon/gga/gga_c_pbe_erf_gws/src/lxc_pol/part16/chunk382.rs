//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 382/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk382(t1235: f64, t19: f64, t299: f64, t799: f64, t119: f64, t331: f64, t391: f64, t4: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1236 = t1235 * t19;
    let t1237 = t799 * t299;
    let t1238 = t1236 * t1237;
    let t1240 = t119 * t331;
    let t1241 = t391 * t1240;
    let t1243 = t4 * t542;
    (t1236, t1237, t1238, t1240, t1241, t1243)
}
