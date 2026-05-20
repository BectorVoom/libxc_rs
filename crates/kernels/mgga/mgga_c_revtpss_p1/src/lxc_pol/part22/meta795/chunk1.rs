//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2892/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2892<F: Float>(t136: F, t9941: F, t1386: F, t820: F, t9948: F, t1401: F, t159: F, t216: F, t4010: F, t2482: F, t2668: F) -> (F, F, F, F, F) {
    let t46716 = t9941 * t136;
    let t46722 = t820 * t1386 * t9948;
    let t46723 = t46722 * t1401;
    let t46730 = t216 * t159 * t4010;
    let t46740 = t2482 * t1386 * t2668;
    (t46716, t46722, t46723, t46730, t46740)
}
