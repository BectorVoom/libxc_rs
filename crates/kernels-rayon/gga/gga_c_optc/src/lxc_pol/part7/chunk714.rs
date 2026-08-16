//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 714/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk714(t2042: f64, t592: f64, t6449: f64, t6457: f64, t6465: f64, t6477: f64, t6741: f64, t6744: f64, t6747: f64, t6750: f64, t6753: f64, t6771: f64, t6773: f64) -> (f64, f64) {
    let t6811 = 60.0_f64 * t2042 * t592;
    let t6812 = t6449 + t6457 + t6741 + t6744 - t6747 - t6750 + t6753 + t6465 + t6771 + t6773 + t6811 + t6477;
    (t6811, t6812)
}
