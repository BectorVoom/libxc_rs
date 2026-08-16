//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1013/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1013(t108: f64, t117: f64, t22154: f64, t56: f64, t127: f64, t616: f64, t6867: f64, t2034: f64, t2030: f64, t6933: f64, t6: f64, t9771: f64) -> (f64, f64, f64, f64, f64) {
    let t22158 = 455.0_f64 / 243.0_f64 * t108 * t22154 * t56 * t117;
    let t22160 = t6867 * t127 * t616;
    let t22161 = t2034 * t22160;
    let t22164 = t2030 * t6933;
    let t22166 = t9771 * t6;
    (t22158, t22160, t22161, t22164, t22166)
}
