//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1714/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1714<F: Float>(t1419: F, t9990: F, t1398: F, t2782: F, t4066: F, t4086: F, t543: F, t10069: F, t10079: F, t3923: F, t4089: F, t40921: F) -> (F, F, F, F, F, F) {
    let t46554 = t9990 * t1419;
    let t46561 = t2782 * t4086 * t4066 * t1398 * t543;
    let t46563 = t10069 * t10079;
    let t46565 = t1419 * t3923;
    let t46568 = t2782 * t4086 * t46565 * t543;
    let t46570 = t40921 * t4089;
    (t46554, t46561, t46563, t46565, t46568, t46570)
}
