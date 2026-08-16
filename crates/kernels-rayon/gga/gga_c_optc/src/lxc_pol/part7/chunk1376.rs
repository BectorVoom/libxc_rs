//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1376/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1376(t3127: f64, t9117: f64, t8196: f64, t9057: f64, t2860: f64, t2850: f64, t25560: f64, t4456: f64, t1113: f64, t8914: f64, t1028: f64, t1179: f64, t27137: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27366 = t9117 * t3127;
    let t27370 = t8196 * t9057;
    let t27374 = t9057 * t2860;
    let t27378 = t9057 * t2850;
    let t27382 = t4456 * t25560;
    let t27383 = t1113 * t8914;
    let t27385 = t8196 * t1028;
    let t27389 = t1179 * t27137;
    (t27366, t27370, t27374, t27378, t27382, t27383, t27385, t27389)
}
