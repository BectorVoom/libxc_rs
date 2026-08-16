//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2953/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2953<F: Float>(t2661: F, t3938: F, t3992: F, t48533: F, t14045: F, t9810: F, t13774: F, t1399: F, t13927: F, t48100: F, t9816: F, t13910: F) -> (F, F, F, F, F) {
    let t48536 = t2661 * t3992 * t48533 * t3938;
    let t48540 = t2661 * t3992 * t14045 * t9810;
    let t48544 = t2661 * t3992 * t13774 * t1399;
    let t48548 = t9816 * t48100 * t13927;
    let t48553 = t2661 * t3992 * t13910 * t1399;
    (t48536, t48540, t48544, t48548, t48553)
}
