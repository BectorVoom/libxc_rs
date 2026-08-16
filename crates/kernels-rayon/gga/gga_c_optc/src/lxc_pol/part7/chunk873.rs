//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 873/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk873(t275: f64, t8378: f64, t176: f64, t2548: f64, t8: f64, t191: f64, t2264: f64, t2436: f64, t2566: f64, t960: f64, t2568: f64, t339: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8379 = t8378 * t275;
    let t8381 = t176 * t8379 * sigma0;
    let t8384 = t8 * t2548;
    let t8385 = t8384 * t191;
    let t8386 = t2436 * t2264;
    let t8387 = t8385 * t8386;
    let t8390 = t2566 * t960;
    let t8393 = 1.0_f64 / t2568 / t339;
    (t8381, t8384, t8385, t8386, t8387, t8390, t8393)
}
