//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 932/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk932(t1648: f64, t4893: f64, t5134: f64, t5312: f64, t5530: f64, t5533: f64, t5536: f64, t5540: f64, t4982: f64, t583: f64, t17391: f64, t17394: f64, t17397: f64, t17402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17404 = 16.0_f64 / 15.0_f64 * t1648 * t4893;
    let t17406 = 32.0_f64 / 15.0_f64 * t5312 * t5134;
    let t17408 = 16.0_f64 / 15.0_f64 * t1648 * t5530;
    let t17410 = 32.0_f64 / 15.0_f64 * t1648 * t5533;
    let t17412 = 16.0_f64 / 9.0_f64 * t1648 * t5536;
    let t17414 = 32.0_f64 / 9.0_f64 * t5312 * t5540;
    let t17415 = t4982 * t583;
    let t17416 = 16.0_f64 / 45.0_f64 * t17415;
    let t17417 = t17391 + t17394 + t17397 + t17402 - t17404 + t17406 - t17408 - t17410 + t17412 + t17414 + t17416;
    (t17404, t17406, t17408, t17410, t17412, t17414, t17416, t17417)
}
