//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2532/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2532(t46456: f64, t786: f64, t10026: f64, t1398: f64, t268: f64, t4101: f64, t543: f64, t793: f64, t10073: f64, t10084: f64, t555: f64, t9898: f64) -> (f64, f64, f64, f64) {
    let t46457 = t786 * t46456;
    let t46458 = t46457 * t10026;
    let t46463 = t4101 * t268 * t793 * t1398 * t543;
    let t46465 = t10073 * t10084;
    let t46469 = t555 * t9898;
    (t46458, t46463, t46465, t46469)
}
