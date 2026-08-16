//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1121/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1121(t16288: f64, t1924: f64, t193: f64, t16247: f64, t603: f64, t75: f64, t16579: f64, t9412: f64, t9416: f64, t3546: f64, t4744: f64, t1256: f64, t4595: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47906 = t193 * t1924 * t16288;
    let t47938 = t16247 * t75 * t603;
    let t47955 = t9412 * t16579;
    let t47957 = t9416 * t16579;
    let t47989 = t3546 * t4744;
    let t48000 = t4595 * t1256;
    (t47906, t47938, t47955, t47957, t47989, t48000)
}
