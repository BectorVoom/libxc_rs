//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 854/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk854(t21: f64, t5589: f64, t2719: f64, t1041: f64, t1251: f64, t1691: f64, t7093: f64, t11: f64, t7212: f64, t2704: f64, t7097: f64, t1413: f64, t2678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7236 = t21 * t5589;
    let t7237 = t7236 * t2719;
    let t7239 = t1251 * t1041;
    let t7248 = t1691 * t7093;
    let t7249 = t11 * t7248;
    let t7251 = t1691 * t7212;
    let t7252 = t2704 * t7251;
    let t7254 = t1691 * t7097;
    let t7255 = t11 * t7254;
    let t7257 = t2678 * t1413;
    (t7236, t7237, t7239, t7249, t7252, t7255, t7257)
}
