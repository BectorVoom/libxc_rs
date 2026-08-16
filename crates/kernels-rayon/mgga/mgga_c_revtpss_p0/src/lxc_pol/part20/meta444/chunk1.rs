//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1700/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1700(t33: f64, t516: f64, t9615: f64, t3842: f64, t3351: f64, t1348: f64, t3881: f64, t43744: f64, t9357: f64, t9617: f64, t9620: f64, t46325: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t46328 = 1.0_f64 / t516 / t9615 / t33;
    let t46329 = t3842 * t3842;
    let t46335 = t3351 * t3351;
    let t46343 = piecewise3(t34, 0.0_f64, -56.0_f64 / 81.0_f64 * t46328 * t46329 + 16.0_f64 / 9.0_f64 * t9617 * t3842 * t3351 - 2.0_f64 / 3.0_f64 * t3881 * t46335 - 8.0_f64 / 9.0_f64 * t9620 * t9357 + 2.0_f64 / 3.0_f64 * t1348 * t43744);
    let t46345 = t46325 / 2.0_f64 + t46343 / 2.0_f64;
    (t46329, t46335, t46345)
}
