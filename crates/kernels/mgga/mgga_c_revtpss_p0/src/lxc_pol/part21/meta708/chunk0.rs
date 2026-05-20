//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2536/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2536<F: Float>(t1438: F, t40317: F, t10065: F, t10069: F, t2782: F, t4086: F, t46469: F, t543: F, t10084: F, t1398: F, t4066: F, t10079: F) -> (F, F, F, F, F, F) {
    let t46526 = t40317 * t1438;
    let t46536 = t10069 * t10065;
    let t46540 = t2782 * t4086 * t46469 * t543;
    let t46542 = t10069 * t10084;
    let t46561 = t2782 * t4086 * t4066 * t1398 * t543;
    let t46563 = t10069 * t10079;
    (t46526, t46536, t46540, t46542, t46561, t46563)
}
