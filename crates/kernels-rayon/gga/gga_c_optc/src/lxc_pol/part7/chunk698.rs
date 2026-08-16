//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 698/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk698(t1864: f64, t587: f64, t6407: f64, t601: f64, t1874: f64, t539: f64, t544: f64, t1963: f64, t1975: f64, t712: f64, t1906: f64, t75: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6617 = t1864 * t6407 * t587;
    let t6619 = 0.35089340384731224426e1_f64 * t601 * t6617;
    let t6620 = t539 * t1874;
    let t6621 = 24.0_f64 * t6620;
    let t6622 = t544 * t1874;
    let t6623 = 24.0_f64 * t6622;
    let t6624 = t539 * t1963;
    let t6625 = 12.0_f64 * t6624;
    let t6626 = t544 * t1963;
    let t6627 = 12.0_f64 * t6626;
    let t6628 = t712 * t1975;
    let t6632 = t1906 * t75;
    (t6617, t6619, t6621, t6623, t6625, t6627, t6628, t6632)
}
