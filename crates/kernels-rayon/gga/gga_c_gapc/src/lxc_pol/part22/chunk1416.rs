//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1416/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1416(t35210: f64, t35212: f64, t35215: f64, t35217: f64, t35222: f64, t35225: f64, t35228: f64, t35231: f64, t35234: f64, t35203: f64, t37236: f64, t35240: f64) -> (f64, f64) {
    let t37237 = 0.18477280112679442116e-5_f64 * t35210;
    let t37238 = 0.80192315782160920384e-6_f64 * t35212;
    let t37239 = 0.80045999977926802214e-7_f64 * t35215;
    let t37240 = 0.10298285674687440379e-4_f64 * t35217;
    let t37241 = 0.15018333275585850553e-5_f64 * t35222;
    let t37242 = 0.6070699179094394313e-6_f64 * t35225;
    let t37243 = 0.43440462632258606772e-4_f64 * t35228;
    let t37244 = 0.11372686522837130914e-5_f64 * t35231;
    let t37245 = 0.10567613244746075633e-6_f64 * t35234;
    let t37246 = 0.7246363367825880434e-6_f64 * t35203 + t37236 + t37237 - t37238 - t37239 + t37240 - t37241 + t37242 + t37243 + t37244 - t37245;
    let t37249 = 0.40483072916666666669e-4_f64 * t35240;
    (t37246, t37249)
}
