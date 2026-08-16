//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 888/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk888(t1111: f64, t8542: f64, t195: f64, t429: f64, t116: f64, t428: f64, t1093: f64, t2916: f64, t3054: f64, t1102: f64, t2917: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8543 = t1111 * t8542;
    let t8545 = t195 * t429;
    let t8546 = t116 * t8545;
    let t8548 = 5.0_f64 / 1296.0_f64 * t428 * t8546;
    let t8549 = t2916 * t1093;
    let t8550 = t8549 * t3054;
    let t8552 = 0.35089340384731224426e1_f64 * t1102 * t8550;
    let t8553 = t2917 * t1093;
    (t8543, t8545, t8548, t8549, t8550, t8552, t8553)
}
