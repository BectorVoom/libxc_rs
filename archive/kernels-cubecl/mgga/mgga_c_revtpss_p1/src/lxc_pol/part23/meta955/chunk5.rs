//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3187/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3187<F: Float>(t1222: F, t17471: F, t24236: F, t24679: F, t369: F, t467: F, t475: F, t5390: F, t6601: F, t21177: F, t5362: F, t1235: F, t127: F, t24634: F, t371: F) -> (F, F, F, F, F) {
    let t83719 = t1222 * t17471 * t24236;
    let t83725 = t467 * t475 * t24679 * t369;
    let t83728 = t6601 * t5390;
    let t83731 = t21177 * t5362;
    let t83735 = t1235 * t371 * t127 * t24634;
    (t83719, t83725, t83728, t83731, t83735)
}
