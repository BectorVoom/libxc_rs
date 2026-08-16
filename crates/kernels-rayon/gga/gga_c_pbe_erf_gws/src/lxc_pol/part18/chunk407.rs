//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 407/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk407(t1430: f64, t409: f64, t428: f64, t1: f64, t427: f64, t467: f64, t408: f64, t413: f64, t88: f64, t119: f64, t331: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1431 = 8.0_f64 * t1430;
    let t1432 = t409 * t428;
    let t1434 = t427 * t1;
    let t1435 = t1434 * t467;
    let t1438 = t408 * t413;
    let t1439 = t1438 * t88;
    let t1440 = 32.0_f64 * t1439;
    let t1444 = t119 * t331 * t84;
    (t1431, t1432, t1434, t1435, t1438, t1440, t1444)
}
