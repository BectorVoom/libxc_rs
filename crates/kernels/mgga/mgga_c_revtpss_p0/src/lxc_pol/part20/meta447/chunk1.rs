//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1707/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1707<F: Float>(t10023: F, t268: F, t4003: F, t46445: F, t10014: F, t10119: F, t1419: F, t5744: F, t786: F, t10026: F, t1398: F, t4101: F, t543: F, t793: F) -> (F, F, F, F) {
    let t46452 = t10023 * t268 * t46445 * t4003;
    let t46454 = t10014 * t10119;
    let t46456 = t5744 * t1419;
    let t46457 = t786 * t46456;
    let t46458 = t46457 * t10026;
    let t46463 = t4101 * t268 * t793 * t1398 * t543;
    (t46452, t46454, t46458, t46463)
}
