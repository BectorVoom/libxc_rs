//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 932/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk932<F: Float>(t1874: F, t2045: F, t539: F, t6830: F, t538: F, t6325: F, t88: F, t1859: F, t1847: F, t587: F, t601: F, t40: F, t558: F, t6524: F, t2193: F, t616: F) -> (F, F, F, F, F, F, F) {
    let t22068 = t2045 * t1874;
    let t22069 = 144.0 * t22068;
    let t22070 = t539 * t6830;
    let t22071 = 48.0 * t22070;
    let t22073 = t538 * t6325 * t88;
    let t22074 = 1920.0 * t22073;
    let t22075 = t1859 * t1859;
    let t22079 = 0.35089340384731224426e1 * t601 * t1847 * t22075 * t587;
    let t22081 = t40 * t558 * t6524;
    let t22082 = 4.0 * t22081;
    let t22086 = t616 * t2193;
    (t22069, t22071, t22074, t22075, t22079, t22082, t22086)
}
