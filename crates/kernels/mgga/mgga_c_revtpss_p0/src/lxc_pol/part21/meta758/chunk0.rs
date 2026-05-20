//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2666/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2666<F: Float>(t13800: F, t46670: F, t3964: F, t5617: F, t9732: F, t136: F, t216: F, t9747: F, t14230: F, t46802: F, t49068: F, t46888: F, t48908: F) -> (F, F, F, F, F) {
    let t49087 = t46670 * t13800;
    let t49090 = t3964 * t9732 * t5617;
    let t49093 = t216 * t9747 * t136;
    let t49103 = t46802 * t49068 * t14230;
    let t49105 = t46888 * t48908;
    (t49087, t49090, t49093, t49103, t49105)
}
