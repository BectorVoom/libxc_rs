//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1715/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1715(t10073: f64, t10079: f64, t46477: f64, t543: f64, t2782: f64, t4003: f64, t46469: f64, t5744: f64, t4086: f64, t46394: f64, t3829: f64, t4010: f64) -> (f64, f64, f64, f64, f64) {
    let t46572 = t10073 * t10079;
    let t46574 = t46477 * t543;
    let t46583 = t2782 * t5744 * t46469 * t4003;
    let t46587 = t2782 * t4086 * t46394 * t543;
    let t46590 = t4010 * t3829;
    (t46572, t46574, t46583, t46587, t46590)
}
