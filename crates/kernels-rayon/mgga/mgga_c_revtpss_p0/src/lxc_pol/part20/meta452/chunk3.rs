//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1727/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1727(t10111: f64, t1408: f64, t9720: f64, t1353: f64, t1414: f64, t685: f64, t9770: f64, t9775: f64, t2661: f64, t3992: f64, t46610: f64, t543: f64) -> (f64, f64, f64, f64) {
    let t46784 = t10111 * t1408 * t9720;
    let t46786 = t1414 * t685 * t1353;
    let t46787 = t46784 * t46786;
    let t46789 = t9775 * t9770;
    let t46793 = t2661 * t3992 * t46610 * t543;
    (t46786, t46787, t46789, t46793)
}
