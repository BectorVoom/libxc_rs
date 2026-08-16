//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2891/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2891<F: Float>(t794: F, t9747: F, t2699: F, t3943: F, t3946: F, t3995: F, t40690: F, t9775: F, t9936: F, t3970: F, t9779: F, t9765: F, t9923: F) -> (F, F, F, F, F, F, F) {
    let t46691 = t794 * t9747;
    let t46694 = t2699 * t3943;
    let t46695 = t46694 * t3946;
    let t46702 = t40690 * t3995;
    let t46704 = t9775 * t9936;
    let t46706 = t9779 * t3970;
    let t46712 = t9765 * t9923;
    (t46691, t46694, t46695, t46702, t46704, t46706, t46712)
}
