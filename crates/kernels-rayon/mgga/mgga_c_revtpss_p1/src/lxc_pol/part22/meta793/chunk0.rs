//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2887/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2887(t10103: f64, t1432: f64, t2470: f64, t3999: f64, t4066: f64, t1438: f64, t40317: f64, t10065: f64, t10069: f64, t10084: f64, t10079: f64, t4089: f64, t40921: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46520 = t1432 * t10103 * t2470;
    let t46522 = t3999 * t4066;
    let t46526 = t40317 * t1438;
    let t46536 = t10069 * t10065;
    let t46542 = t10069 * t10084;
    let t46563 = t10069 * t10079;
    let t46570 = t40921 * t4089;
    (t46520, t46522, t46526, t46536, t46542, t46563, t46570)
}
