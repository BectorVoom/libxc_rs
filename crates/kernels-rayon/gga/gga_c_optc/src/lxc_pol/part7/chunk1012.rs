//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1012/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1012(t1986: f64, t6636: f64, t6642: f64, t539: f64, t6828: f64, t544: f64, t1846: f64, t1863: f64, t22120: f64, t601: f64, t6427: f64, t2040: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22133 = t1986 * t6636;
    let t22134 = 0.41015588084031179722e4_f64 * t22133;
    let t22135 = t1986 * t6642;
    let t22136 = 0.23392893589820816284e1_f64 * t22135;
    let t22140 = t539 * t6828;
    let t22141 = 16.0_f64 * t22140;
    let t22142 = t544 * t6828;
    let t22143 = 16.0_f64 * t22142;
    let t22148 = 1.0_f64 / t1863 / t1846;
    let t22152 = 0.12304676425209353917e5_f64 * t601 * t22148 * t22120 * t6427;
    let t22154 = 1.0_f64 / t8 / t2040;
    (t22134, t22136, t22141, t22143, t22148, t22152, t22154)
}
