//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2952/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2952<F: Float>(t14055: F, t9775: F, t1885: F, t46722: F, t13867: F, t221: F, t3978: F, t9921: F, t14047: F, t14051: F, t1412: F, t5658: F) -> (F, F, F, F, F, F) {
    let t48516 = t9775 * t14055;
    let t48518 = t46722 * t1885;
    let t48525 = t221 * t13867;
    let t48527 = t3978 * t9921 * t48525;
    let t48529 = t9775 * t14047;
    let t48531 = t9775 * t14051;
    let t48533 = t1412 * t5658;
    (t48516, t48518, t48527, t48529, t48531, t48533)
}
