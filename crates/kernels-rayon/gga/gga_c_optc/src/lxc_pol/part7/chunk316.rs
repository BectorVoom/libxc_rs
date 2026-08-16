//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 316/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk316(t1008: f64, t883: f64, t1007: f64, t6: f64, t8: f64, t191: f64, t56: f64, t362: f64, t287: f64) -> (f64, f64, f64, f64, f64) {
    let t1009 = t1008 * t883;
    let t1010 = t1007 * t1009;
    let t1011 = t6 * t8;
    let t1013 = t191 * t56;
    let t1014 = t1013 * t362;
    let t1015 = t1011 * t287 * t1014;
    (t1009, t1010, t1013, t1014, t1015)
}
