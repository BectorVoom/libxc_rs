//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1638/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1638<F: Float>(t126: F, t13099: F, t12257: F, t1261: F, t247: F, t12879: F, t3372: F, t3368: F, t1222: F, t12287: F, t17240: F, t12881: F, t3647: F) -> (F, F, F, F, F) {
    let t44895 = t126 * t13099;
    let t44898 = t1261 * t247 * t44895 * t12257;
    let t44902 = t1261 * t247 * t12879 * t3372;
    let t44906 = t1261 * t247 * t12879 * t3368;
    let t44912 = t1222 * t17240 * t12287;
    let t44917 = t3647 * t12881;
    (t44898, t44902, t44906, t44912, t44917)
}
