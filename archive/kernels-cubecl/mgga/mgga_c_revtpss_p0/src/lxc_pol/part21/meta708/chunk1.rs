//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2537/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2537<F: Float>(t1419: F, t3923: F, t2782: F, t4086: F, t543: F, t4089: F, t40921: F, t10073: F, t10079: F, t4003: F, t46469: F, t5744: F) -> (F, F, F, F, F) {
    let t46565 = t1419 * t3923;
    let t46568 = t2782 * t4086 * t46565 * t543;
    let t46570 = t40921 * t4089;
    let t46572 = t10073 * t10079;
    let t46583 = t2782 * t5744 * t46469 * t4003;
    (t46565, t46568, t46570, t46572, t46583)
}
