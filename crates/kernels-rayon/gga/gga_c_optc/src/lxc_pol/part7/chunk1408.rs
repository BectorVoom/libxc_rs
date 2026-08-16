//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1408/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1408(t1210: f64, t9227: f64, t1214: f64, t9340: f64, t3268: f64, t26181: f64, t26184: f64, t26188: f64, t26192: f64, t26200: f64, t26203: f64, t26206: f64, t26209: f64, t26212: f64, t277: f64, t2911: f64, t95: f64) -> f64 {
    let t28040 = t1210 * t9227;
    let t28042 = t9340 * t1214;
    let t28044 = t3268 * t3268;
    let t28049 = -t26181 - t26184 - t26188 + t26192 + 20.0_f64 / 27.0_f64 * t28040 + 2.0_f64 / 3.0_f64 * t28042 + t26200 - t26203 - t26206 + t26209 + t26212 - 0.77534644304710291488e-2_f64 * t95 * t277 * t28044 * t2911;
    t28049
}
