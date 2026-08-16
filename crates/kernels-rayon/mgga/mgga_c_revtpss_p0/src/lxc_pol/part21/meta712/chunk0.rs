//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2544/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2544(t3946: f64, t46694: f64, t3995: f64, t40690: f64, t9775: f64, t9936: f64, t3970: f64, t9779: f64, t9765: f64, t9923: f64, t136: f64, t9941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46695 = t46694 * t3946;
    let t46702 = t40690 * t3995;
    let t46704 = t9775 * t9936;
    let t46706 = t9779 * t3970;
    let t46712 = t9765 * t9923;
    let t46716 = t9941 * t136;
    (t46695, t46702, t46704, t46706, t46712, t46716)
}
