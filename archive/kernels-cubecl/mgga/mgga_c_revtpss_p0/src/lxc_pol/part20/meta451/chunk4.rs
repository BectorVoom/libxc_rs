//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1721/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1721<F: Float>(t125: F, t9890: F, t794: F, t9747: F, t9750: F, t2699: F, t3943: F, t3946: F, t3995: F, t40690: F, t9775: F, t9936: F) -> (F, F, F, F, F) {
    let t46682 = t125 * t9890;
    let t46691 = t794 * t9747;
    let t46692 = t46691 * t9750;
    let t46694 = t2699 * t3943;
    let t46695 = t46694 * t3946;
    let t46702 = t40690 * t3995;
    let t46704 = t9775 * t9936;
    (t46682, t46692, t46695, t46702, t46704)
}
