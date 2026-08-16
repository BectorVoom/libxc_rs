//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1190/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1190(t2655: f64, t7416: f64, t2608: f64, t2619: f64, t874: f64, t2658: f64, t7421: f64, t7907: f64, t858: f64, t7365: f64, t224: f64, t2263: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24636 = t2655 * t7416;
    let t24644 = t874 * t2619 * t2608;
    let t24646 = t7421 * t2658;
    let t24652 = t7907 * t858;
    let t24654 = t2655 * t7365;
    let t24657 = 1.0_f64 / t224 / t2263;
    (t24636, t24644, t24646, t24652, t24654, t24657)
}
