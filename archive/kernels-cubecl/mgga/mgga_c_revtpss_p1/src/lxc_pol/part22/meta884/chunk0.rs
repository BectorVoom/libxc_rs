//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3058/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3058<F: Float>(t2439: F, t4622: F, t15186: F, t698: F, t15177: F, t15180: F, t15162: F, t15165: F, t123: F, t127: F, t159: F) -> (F, F, F, F, F, F, F) {
    let t51915 = t2439 * t4622;
    let t51917 = t698 * t15186;
    let t51921 = t698 * t15177;
    let t51923 = t698 * t15180;
    let t51937 = t698 * t15162;
    let t51942 = t698 * t15165;
    let t51957 = t123 * t127 * t159;
    (t51915, t51917, t51921, t51923, t51937, t51942, t51957)
}
