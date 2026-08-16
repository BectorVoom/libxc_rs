//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1382/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1382(t3127: f64, t9123: f64, t3234: f64, t9040: f64, t9189: f64, t3244: f64, t9058: f64, t9142: f64, t1113: f64, t530: f64, t3237: f64, t11899: f64, t3105: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27493 = t9123 * t3127;
    let t27510 = t3234 * t9189 * t9040;
    let t27513 = t3244 * t9142 * t9058;
    let t27515 = t530 * t1113;
    let t27517 = t3244 * t27515 * t3237;
    let t27528 = t3244 * t9142 * t9040;
    let t27533 = t11899 * t3105;
    (t27493, t27510, t27513, t27517, t27528, t27533)
}
