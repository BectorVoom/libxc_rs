//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 946/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk946(t41588: f64, t41592: f64, t41594: f64, t41606: f64, t41615: f64, t41621: f64, t41631: f64, t41636: f64, t41640: f64, t41643: f64, t41645: f64, t46030: f64, t46031: f64, t46033: f64, t46035: f64, t46036: f64, t46037: f64, t46045: f64, t46047: f64) -> f64 {
    let t46049 = 0.38342925953920749677e1_f64 * t41588 - 0.23005755572352449806e1_f64 * t41592 - 0.51123901271894332903e1_f64 * t41594 - 0.38342925953920749677e1_f64 * t41606 - t46030 + t46031 + 0.63904876589867916128e-1_f64 * t41615 - t46033 + 0.11916829983950142223e0_f64 * t41621 + t46035 + t46036 + t46037 + 0.76685851907841499353e0_f64 * t41631 + 0.76685851907841499353e0_f64 * t41636 - 0.17041300423964777634e0_f64 * t41640 - 0.59584149919750711116e-1_f64 * t41643 + 0.38342925953920749677e1_f64 * t41645 + t46045 + 0.38342925953920749677e0_f64 * t46047;
    t46049
}
