//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2880/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2880(t3853: f64, t3860: f64, t30: f64, t513: f64, t9603: f64, t33: f64, t516: f64, t9615: f64, t10153: f64, t2435: f64, t2439: f64, t3895: f64, t4078: f64) -> (f64, f64, f64, f64, f64) {
    let t46302 = t3860 * t3853;
    let t46310 = 1.0_f64 / t513 / t9603 / t30;
    let t46328 = 1.0_f64 / t516 / t9615 / t33;
    let t46353 = t2435 * t10153;
    let t46356 = t2439 * t3895 * t4078;
    (t46302, t46310, t46328, t46353, t46356)
}
