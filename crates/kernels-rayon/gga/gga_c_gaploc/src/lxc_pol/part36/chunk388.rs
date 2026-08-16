//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 388/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk388(t3221: f64, t3220: f64, t3091: f64, t713: f64, t928: f64, t3217: f64, t3218: f64, t871: f64, t931: f64, t295: f64, t3113: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3222 = t3221 * pi;
    let t3223 = t3220 * t3222;
    let t3225 = t713 * t3091;
    let t3226 = t3225 * t928;
    let t3227 = t3226 / 256.0_f64;
    let t3228 = t3217 - 9.0_f64 / 8192.0_f64 * t3218 + 3.0_f64 / 8192.0_f64 * t3223 - t3227;
    let t3230 = t931 * t871;
    let t3232 = t295 * t3113;
    let t3234 = t3228 * t471 + t3230 / 2.0_f64 + t3217 - t3227 - t3232 / 2.0_f64;
    (t3222, t3223, t3225, t3226, t3228, t3232, t3234)
}
