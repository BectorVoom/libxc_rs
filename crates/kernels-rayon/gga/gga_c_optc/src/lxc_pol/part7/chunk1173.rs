//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1173/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1173(t24320: f64, t24333: f64, t265: f64, t241: f64, t2441: f64, t7202: f64, t2466: f64, t7504: f64, t2473: f64, t7501: f64, t845: f64, t2248: f64, t7207: f64) -> (f64, f64, f64, f64, f64) {
    let t24335 = (t24320 + t24333) * t265;
    let t24337 = 0.19751789702565206229e-1_f64 * t241 * t24335;
    let t24339 = 0.14035736153892489771e2_f64 * t2441 * t7202;
    let t24341 = t7504 * t2466;
    let t24344 = 0.61523382126046769581e4_f64 * t845 * t7501 * t2473 * t24341;
    let t24345 = t2248 * t7207;
    (t24335, t24337, t24339, t24344, t24345)
}
