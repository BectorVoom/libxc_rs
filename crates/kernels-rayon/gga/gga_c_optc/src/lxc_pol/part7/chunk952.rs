//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 952/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk952(t9062: f64, t9074: f64, t8483: f64, t914: f64, t8533: f64, t2367: f64, t3097: f64, t1162: f64, t3088: f64, t1781: f64, t321: f64, t429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9075 = t9062 * t9074;
    let t9078 = t914 * t8483;
    let t9081 = t914 * t8533;
    let t9084 = t2367 * t3097;
    let t9085 = t1162 * t9084;
    let t9087 = t2367 * t3088;
    let t9088 = t1162 * t9087;
    let t9091 = t321 * t1781 * t429;
    (t9075, t9078, t9081, t9085, t9088, t9091)
}
