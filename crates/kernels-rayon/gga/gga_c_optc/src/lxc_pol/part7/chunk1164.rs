//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1164/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1164(t4038: f64, t7179: f64, t8152: f64, t7174: f64, t7213: f64, t2433: f64, t870: f64, t981: f64, t2360: f64, t7294: f64, t23817: f64, t2529: f64, t837: f64, t845: f64) -> (f64, f64, f64, f64, f64) {
    let t24187 = t4038 * t8152 * t7179;
    let t24189 = t7213 * t7174;
    let t24190 = t2433 * t24189;
    let t24192 = t981 * t870;
    let t24197 = t2360 * t7294;
    let t24202 = 0.35089340384731224426e1_f64 * t845 * t2529 * t23817 * t837;
    (t24187, t24190, t24192, t24197, t24202)
}
