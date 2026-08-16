//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1712/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1712(t3999: f64, t4066: f64, t1438: f64, t40317: f64, t4056: f64, t543: f64, t10065: f64, t10069: f64, t2782: f64, t4086: f64, t46469: f64, t10084: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46522 = t3999 * t4066;
    let t46526 = t40317 * t1438;
    let t46531 = t4056 * t4056;
    let t46532 = t46531 * t543;
    let t46536 = t10069 * t10065;
    let t46540 = t2782 * t4086 * t46469 * t543;
    let t46542 = t10069 * t10084;
    (t46522, t46526, t46531, t46532, t46536, t46540, t46542)
}
