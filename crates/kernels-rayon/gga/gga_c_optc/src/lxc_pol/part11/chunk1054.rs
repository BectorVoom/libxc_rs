//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1054/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1054(t25423: f64, t9168: f64, t1027: f64, t19: f64, t1113: f64, t530: f64, t4434: f64, t3181: f64, t442: f64, t462: f64, t27173: f64, t3102: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27438 = t9168 * t25423;
    let t27441 = t19 * t1027;
    let t27515 = t530 * t1113;
    let t27552 = t4434 * t25423;
    let t27629 = 1.0_f64 / t3181 / t462 * t442;
    let t27630 = t27629 * t27173;
    let t27644 = t3102 * t25423;
    (t27438, t27441, t27515, t27552, t27630, t27644)
}
