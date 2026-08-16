//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1387/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1387<F: Float>(t794: F, t9747: F, t2699: F, t3943: F, t136: F, t9941: F, t1386: F, t820: F, t9948: F, t159: F, t216: F, t4010: F) -> (F, F, F, F, F) {
    let t46691 = t794 * t9747;
    let t46694 = t2699 * t3943;
    let t46716 = t9941 * t136;
    let t46722 = t820 * t1386 * t9948;
    let t46730 = t216 * t159 * t4010;
    (t46691, t46694, t46716, t46722, t46730)
}
