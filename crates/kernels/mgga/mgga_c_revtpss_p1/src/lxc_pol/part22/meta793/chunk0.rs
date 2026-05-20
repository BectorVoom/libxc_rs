//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2887/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2887<F: Float>(t10103: F, t1432: F, t2470: F, t3999: F, t4066: F, t1438: F, t40317: F, t10065: F, t10069: F, t10084: F, t10079: F, t4089: F, t40921: F) -> (F, F, F, F, F, F, F) {
    let t46520 = t1432 * t10103 * t2470;
    let t46522 = t3999 * t4066;
    let t46526 = t40317 * t1438;
    let t46536 = t10069 * t10065;
    let t46542 = t10069 * t10084;
    let t46563 = t10069 * t10079;
    let t46570 = t40921 * t4089;
    (t46520, t46522, t46526, t46536, t46542, t46563, t46570)
}
