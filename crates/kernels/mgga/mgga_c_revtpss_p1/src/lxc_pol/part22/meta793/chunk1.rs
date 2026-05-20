//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2888/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2888<F: Float>(t10073: F, t10079: F, t3829: F, t4010: F, t808: F, t9736: F, t1408: F, t820: F, t9948: F, t1416: F, t9775: F, t9931: F) -> (F, F, F, F, F) {
    let t46572 = t10073 * t10079;
    let t46590 = t4010 * t3829;
    let t46592 = t9736 * t808 * t46590;
    let t46595 = t820 * t1408 * t9948;
    let t46596 = t46595 * t1416;
    let t46598 = t9775 * t9931;
    (t46572, t46592, t46595, t46596, t46598)
}
