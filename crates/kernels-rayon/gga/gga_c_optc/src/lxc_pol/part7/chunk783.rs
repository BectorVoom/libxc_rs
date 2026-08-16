//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 783/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk783(t7342: f64, t7501: f64, t7504: f64, t845: f64, t2441: f64, t2468: f64, t217: f64, t226: f64, t2383: f64, t782: f64, t2382: f64, t2391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7505 = t7501 * t7342 * t7504;
    let t7507 = 0.1025389702100779493e4_f64 * t845 * t7505;
    let t7509 = 0.17544670192365612213e1_f64 * t2441 * t2468;
    let t7512 = 1.0_f64 / t217 / t226 / 4.0_f64;
    let t7513 = t2383 * t782;
    let t7514 = t7512 * t7513;
    let t7516 = t2382 * t782;
    let t7517 = t7516 * t2391;
    (t7505, t7507, t7509, t7512, t7513, t7514, t7516, t7517)
}
