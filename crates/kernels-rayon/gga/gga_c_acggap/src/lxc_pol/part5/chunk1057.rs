//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1057/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1057(t1165: f64, t14373: f64, t1532: f64, t301: f64, t4183: f64, t12813: f64, t5286: f64, t1163: f64, t1181: f64, t14575: f64, t535: f64, t1552: f64, t3451: f64, t372: f64) -> (f64, f64, f64, f64) {
    let t18605 = t14373 * t1165 * t1532 * t4183 * t301;
    let t18607 = t12813 * t5286;
    let t18611 = t1163 * t1181 * t535 * t14575;
    let t18616 = t3451 * t1165 * t1552 * t4183 * t372;
    (t18605, t18607, t18611, t18616)
}
