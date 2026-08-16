//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2346/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2346(t12988: f64, t13005: f64, t16771: f64, t20756: f64, t20800: f64, t213: f64, t221: f64, t4119: f64, t41200: f64, t4127: f64, t46770: f64, t46772: f64, t46783: f64, t46847: f64, t5544: f64, t59154: f64, t59156: f64, t59165: f64, t59173: f64, t776: f64) -> f64 {
    let t68102 = 0.13999999999999999999e0_f64 * t59154 - 0.69999999999999999996e-1_f64 * t59156 + 0.29999999999999999999e-1_f64 * t59165 - 0.14999999999999999999e-1_f64 * t59173 - 0.38888888888888888888e-1_f64 * t46770 + 0.98611111111111111109e-1_f64 * t46772 - t46783 - t41200 + 0.49999999999999999998e-2_f64 * t4127 * t221 * t213 * t20800 * t776 + 0.99999999999999999995e-1_f64 * t46847 * t221 * t213 * t20756 * t776 - 0.59999999999999999997e-1_f64 * t13005 * t221 * t16771 * t4119 + 0.14999999999999999999e-1_f64 * t4127 * t221 * t12988 * t5544;
    t68102
}
