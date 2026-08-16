//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2537/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2537(t1419: f64, t3923: f64, t2782: f64, t4086: f64, t543: f64, t4089: f64, t40921: f64, t10073: f64, t10079: f64, t4003: f64, t46469: f64, t5744: f64) -> (f64, f64, f64, f64, f64) {
    let t46565 = t1419 * t3923;
    let t46568 = t2782 * t4086 * t46565 * t543;
    let t46570 = t40921 * t4089;
    let t46572 = t10073 * t10079;
    let t46583 = t2782 * t5744 * t46469 * t4003;
    (t46565, t46568, t46570, t46572, t46583)
}
