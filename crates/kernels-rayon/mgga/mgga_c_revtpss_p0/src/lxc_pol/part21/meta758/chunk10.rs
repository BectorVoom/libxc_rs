//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2676/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2676(t13920: f64, t555: f64, t14122: f64, t14171: f64, t46433: f64, t46536: f64, t46540: f64, t46542: f64, t46561: f64, t46563: f64, t46568: f64, t46570: f64, t5675: f64, t5735: f64, t5745: f64, t5755: f64, t820: f64, t9840: f64, t9912: f64) -> (f64, f64) {
    let t49213 = t555 * t13920;
    let t49233 = 0.39512695097613069591e1_f64 * t5745 * t49213 * t5675 - 0.21951497276451705329e-1_f64 * t46536 + 0.54878743191129263322e-2_f64 * t46540 - 0.21951497276451705329e-1_f64 * t46542 + 0.39512695097613069591e1_f64 * t820 * t14171 * t9912 + 0.39512695097613069591e1_f64 * t5745 * t14122 * t9840 - 0.19756347548806534796e1_f64 * t5755 * t5735 * t46433 + 0.16463622957338778996e-1_f64 * t46561 - 0.43902994552903410657e-1_f64 * t46563 + 0.16463622957338778996e-1_f64 * t46568 + 0.51220160311720645767e-1_f64 * t46570;
    (t49213, t49233)
}
