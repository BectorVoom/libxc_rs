//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1075/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1075<F: Float>(t15512: F, t332: F, t442: F, t6: F, t7877: F, t2763: F, t3326: F, t2456: F, t3787: F, t7521: F, t6851: F, t871: F) -> (F, F, F, F, F, F, F) {
    let t15513 = t15512 * t332;
    let t15515 = t6 * t7877 * t442;
    let t15516 = t15513 * t15515;
    let t15541 = t3326 * t2763;
    let t15542 = t15541 * t2456;
    let t15548 = t3787 * t7521;
    let t15553 = t871 * t6851;
    (t15513, t15515, t15516, t15541, t15542, t15548, t15553)
}
