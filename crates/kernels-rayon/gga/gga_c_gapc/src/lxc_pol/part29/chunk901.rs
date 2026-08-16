//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 901/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk901(t1423: f64, t1464: f64, t3651: f64, t632: f64, t996: f64, t3634: f64, t458: f64, t568: f64, t997: f64, t437: f64, t516: f64, t8356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11185 = t1423 * t1464;
    let t11186 = t3651 * t11185;
    let t11188 = t996 * t632;
    let t11189 = t3634 * t458;
    let t11190 = t11188 * t11189;
    let t11192 = t3634 * t568;
    let t11193 = t997 * t11192;
    let t11195 = t516 * t437;
    let t11196 = t8356 * t11195;
    (t11185, t11186, t11188, t11189, t11190, t11192, t11193, t11195, t11196)
}
