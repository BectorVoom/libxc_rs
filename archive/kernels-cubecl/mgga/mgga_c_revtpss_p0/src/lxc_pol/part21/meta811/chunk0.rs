//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2962/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2962<F: Float>(t15711: F, t3188: F, t1011: F, t15145: F, t15987: F, t15149: F, t15154: F, t15993: F, t15130: F, t15135: F, t11821: F, t140: F) -> (F, F, F, F, F, F, F) {
    let t53955 = t3188 * t15711;
    let t53958 = t1011 * t15987 * t15145;
    let t53961 = t1011 * t15987 * t15149;
    let t53964 = t1011 * t15993 * t15154;
    let t53967 = t1011 * t15993 * t15130;
    let t53970 = t1011 * t15993 * t15135;
    let t53972 = t140 * t11821;
    (t53955, t53958, t53961, t53964, t53967, t53970, t53972)
}
