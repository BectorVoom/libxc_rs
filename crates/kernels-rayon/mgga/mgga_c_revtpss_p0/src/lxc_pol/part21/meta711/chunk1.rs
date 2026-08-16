//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2543/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2543(t46670: f64, t9738: f64, t124: f64, t3938: f64, t4056: f64, t9816: f64, t9818: f64, t794: f64, t9747: f64, t9750: f64, t2699: f64, t3943: f64) -> (f64, f64, f64, f64, f64) {
    let t46671 = t46670 * t9738;
    let t46680 = t9816 * t9818 * t124 * t4056 * t3938;
    let t46691 = t794 * t9747;
    let t46692 = t46691 * t9750;
    let t46694 = t2699 * t3943;
    (t46671, t46680, t46691, t46692, t46694)
}
