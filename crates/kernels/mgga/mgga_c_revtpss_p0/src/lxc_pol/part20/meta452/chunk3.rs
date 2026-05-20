//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1727/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1727<F: Float>(t10111: F, t1408: F, t9720: F, t1353: F, t1414: F, t685: F, t9770: F, t9775: F, t2661: F, t3992: F, t46610: F, t543: F) -> (F, F, F, F) {
    let t46784 = t10111 * t1408 * t9720;
    let t46786 = t1414 * t685 * t1353;
    let t46787 = t46784 * t46786;
    let t46789 = t9775 * t9770;
    let t46793 = t2661 * t3992 * t46610 * t543;
    (t46786, t46787, t46789, t46793)
}
