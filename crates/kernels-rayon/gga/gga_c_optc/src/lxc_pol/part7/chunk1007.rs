//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1007/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1007(t1874: f64, t2045: f64, t539: f64, t6830: f64, t538: f64, t6325: f64, t88: f64, t1859: f64, t1847: f64, t587: f64, t601: f64, t40: f64, t558: f64, t6524: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22068 = t2045 * t1874;
    let t22069 = 144.0_f64 * t22068;
    let t22070 = t539 * t6830;
    let t22071 = 48.0_f64 * t22070;
    let t22073 = t538 * t6325 * t88;
    let t22074 = 1920.0_f64 * t22073;
    let t22075 = t1859 * t1859;
    let t22079 = 0.35089340384731224426e1_f64 * t601 * t1847 * t22075 * t587;
    let t22081 = t40 * t558 * t6524;
    (t22069, t22071, t22074, t22075, t22079, t22081)
}
