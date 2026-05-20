//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3159/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3159<F: Float>(t17654: F, t17657: F, t56756: F, t247: F, t44545: F, t5230: F, t5384: F, t12984: F, t5327: F, t12995: F, t17438: F, t17303: F, t3667: F) -> (F, F, F, F, F) {
    let t57227 = t17654 * t56756 * t17657;
    let t57241 = t5384 * t247 * t44545 * t5230;
    let t57250 = t5327 * t12984;
    let t57252 = t17438 * t12995;
    let t57256 = t3667 * t17303;
    (t57227, t57241, t57250, t57252, t57256)
}
