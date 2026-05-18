//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1071/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1071<F: Float>(t2404: F, t286: F, t442: F, t8132: F, t670: F, t327: F, t7875: F, t332: F, t6: F, t7877: F, t2763: F, t3326: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15489 = t2404 * t286;
    let t15491 = t8132 * t15489 * t442;
    let t15507 = t670 * t670;
    let t15508 = F::new(1.0) / t15507;
    let t15512 = t327 * t7875;
    let t15513 = t15512 * t332;
    let t15515 = t6 * t7877 * t442;
    let t15516 = t15513 * t15515;
    let t15541 = t3326 * t2763;
    (t15489, t15491, t15507, t15508, t15512, t15513, t15515, t15516, t15541)
}
