//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1288/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1288(t51325: f64, t54341: f64, t14058: f64, t3279: f64, t4049: f64, t9647: f64, t4028: f64, t9009: f64, t9013: f64, t1158: f64, t51395: f64, t3268: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54342 = t54341 * t51325;
    let t54344 = t14058 * t3279;
    let t54346 = t4049 * t9647;
    let t54348 = t4028 * t9009;
    let t54350 = t4028 * t9013;
    let t54352 = t51395 * t1158;
    let t54354 = t14058 * t3268;
    (t54342, t54344, t54346, t54348, t54350, t54352, t54354)
}
