//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 774/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk774(t2619: f64, t877: f64, t874: f64, t140: f64, t2246: f64, t871: f64, t329: f64, t2655: f64, t2658: f64, t2661: f64, t883: f64, t2667: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7416 = t2619 * t877;
    let t7417 = t874 * t7416;
    let t7420 = t2246 * t871 * t140;
    let t7421 = t329 * t7420;
    let t7424 = t2655 * t2658;
    let t7426 = t2661 * t883;
    let t7427 = t7426 * t2667;
    (t7416, t7417, t7420, t7421, t7424, t7426, t7427)
}
