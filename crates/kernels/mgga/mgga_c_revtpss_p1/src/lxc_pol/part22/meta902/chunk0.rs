//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3097/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3097<F: Float>(t1011: F, t15154: F, t15993: F, t15130: F, t15135: F, t11821: F, t140: F, t15140: F, t11710: F, t15614: F, t3091: F, t1063: F, t15937: F, t3172: F) -> (F, F, F, F, F, F, F) {
    let t53964 = t1011 * t15993 * t15154;
    let t53967 = t1011 * t15993 * t15130;
    let t53970 = t1011 * t15993 * t15135;
    let t53972 = t140 * t11821;
    let t53974 = t1011 * t53972 * t15140;
    let t53993 = t3091 * t11710 * t15614;
    let t53998 = t1063 * t3172 * t15937;
    (t53964, t53967, t53970, t53972, t53974, t53993, t53998)
}
