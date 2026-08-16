//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1708/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1708(t10073: f64, t10084: f64, t10059: f64, t10130: f64, t3924: f64, t4057: f64, t46435: f64, t46443: f64, t46448: f64, t46452: f64, t46454: f64, t46458: f64, t46463: f64, t5745: f64, t5755: f64, t820: f64, t9840: f64) -> f64 {
    let t46465 = t10073 * t10084;
    let t46467 = 0.23707617058567841754e2_f64 * t5745 * t10059 * t9840 - 0.11708928647259339623e0_f64 * t46435 - 0.39512695097613069592e1_f64 * t5755 * t10059 * t4057 - 0.39512695097613069592e1_f64 * t820 * t10130 * t3924 + 0.15611904863012452831e0_f64 * t46443 + 0.78059524315062264152e-1_f64 * t46448 - 0.1561190486301245283e0_f64 * t46452 - 0.11708928647259339623e0_f64 * t46454 + 0.23417857294518679245e0_f64 * t46458 - 0.12142592671231907757e0_f64 * t46463 + 0.39029762157531132076e-2_f64 * t46465;
    t46467
}
