//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2437/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2437<F: Float>(t1419: F, t9990: F, t4089: F, t40921: F, t1408: F, t820: F, t9948: F, t1416: F, t240: F, t9991: F, t3995: F, t40488: F) -> (F, F, F, F, F, F) {
    let t46554 = t9990 * t1419;
    let t46570 = t40921 * t4089;
    let t46595 = t820 * t1408 * t9948;
    let t46596 = t46595 * t1416;
    let t46609 = t9991 * t240;
    let t46620 = t40488 * t3995;
    (t46554, t46570, t46595, t46596, t46609, t46620)
}
