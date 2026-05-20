//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1712/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1712<F: Float>(t3999: F, t4066: F, t1438: F, t40317: F, t4056: F, t543: F, t10065: F, t10069: F, t2782: F, t4086: F, t46469: F, t10084: F) -> (F, F, F, F, F, F, F) {
    let t46522 = t3999 * t4066;
    let t46526 = t40317 * t1438;
    let t46531 = t4056 * t4056;
    let t46532 = t46531 * t543;
    let t46536 = t10069 * t10065;
    let t46540 = t2782 * t4086 * t46469 * t543;
    let t46542 = t10069 * t10084;
    (t46522, t46526, t46531, t46532, t46536, t46540, t46542)
}
