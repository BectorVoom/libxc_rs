//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1019/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1019<F: Float>(t1025: F, t15749: F, t1647: F, t3140: F, t3149: F, t1660: F, t3201: F, t11243: F, t72: F, t3088: F) -> (F, F, F, F, F, F) {
    let t15750 = t1025 * t15749;
    let t15822 = t1647 * t3140;
    let t15823 = t15822 * t3149;
    let t15862 = t1660 * t3201;
    let t15904 = t11243 * t72;
    let t15905 = t3088 * t15904;
    (t15750, t15822, t15823, t15862, t15904, t15905)
}
