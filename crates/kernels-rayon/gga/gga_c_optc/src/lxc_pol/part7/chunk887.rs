//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 887/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk887(t1772: f64, t449: f64, t310: f64, t448: f64, t3086: f64, t8414: f64, t6548: f64, t322: f64, t1113: f64, t2849: f64, t24: f64, t3093: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8528 = t1772 * t449;
    let t8529 = t310 * t8528;
    let t8531 = 0.80492236016562572729e-3_f64 * t448 * t8529;
    let t8532 = t3086 * t8414;
    let t8533 = t8532 * t6548;
    let t8534 = t322 * t8533;
    let t8537 = t1113 * t2849;
    let t8538 = t8537 * t6548;
    let t8539 = t322 * t8538;
    let t8542 = t24 * t3093;
    (t8528, t8529, t8531, t8532, t8533, t8534, t8537, t8538, t8539, t8542)
}
