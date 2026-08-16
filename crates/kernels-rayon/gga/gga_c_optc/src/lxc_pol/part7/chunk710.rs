//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 710/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk710(t6772: f64, t104: f64, t188: f64, t6465: f64, t6741: f64, t6744: f64, t6747: f64, t6750: f64, t6753: f64, t6757: f64, t6761: f64, t6763: f64, t6766: f64, t6771: f64, t95: f64) -> (f64, f64) {
    let t6773 = 60.0_f64 * t6772;
    let t6774 = t6741 + t6744 - t6747 - t6750 + t6753 + t188 * t6757 / 2.0_f64 - 7.0_f64 / 2.0_f64 * t6761 + t6465 + 0.51689762869806860992e-2_f64 * t95 * t104 * t6763 * t6766 + t6771 + t6773;
    (t6773, t6774)
}
