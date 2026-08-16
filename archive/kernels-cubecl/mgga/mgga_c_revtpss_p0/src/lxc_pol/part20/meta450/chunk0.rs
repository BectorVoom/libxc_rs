//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1715/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1715<F: Float>(t10073: F, t10079: F, t46477: F, t543: F, t2782: F, t4003: F, t46469: F, t5744: F, t4086: F, t46394: F, t3829: F, t4010: F) -> (F, F, F, F, F) {
    let t46572 = t10073 * t10079;
    let t46574 = t46477 * t543;
    let t46583 = t2782 * t5744 * t46469 * t4003;
    let t46587 = t2782 * t4086 * t46394 * t543;
    let t46590 = t4010 * t3829;
    (t46572, t46574, t46583, t46587, t46590)
}
