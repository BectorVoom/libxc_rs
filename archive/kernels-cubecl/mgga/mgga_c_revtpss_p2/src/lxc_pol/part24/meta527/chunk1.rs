//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1561/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1561<F: Float>(t5390: F, t6601: F, t21177: F, t5362: F, t1235: F, t127: F, t24634: F, t371: F, t20842: F, t5327: F, t17396: F, t20926: F) -> (F, F, F, F, F) {
    let t83728 = t6601 * t5390;
    let t83731 = t21177 * t5362;
    let t83735 = t1235 * t371 * t127 * t24634;
    let t83748 = t5327 * t20842;
    let t83751 = t17396 * t20926;
    (t83728, t83731, t83735, t83748, t83751)
}
