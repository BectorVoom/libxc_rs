//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3083/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3083<F: Float>(t1063: F, t15193: F, t247: F, t3109: F, t11710: F, t15600: F, t3091: F, t127: F, t4823: F, t11774: F, t3096: F, t11670: F, t15687: F) -> (F, F, F, F, F) {
    let t53363 = t1063 * t247 * t3109 * t15193;
    let t53389 = t3091 * t11710 * t15600;
    let t53391 = t127 * t4823;
    let t53393 = t11774 * t53391 * t3096;
    let t53401 = t11670 * t15687;
    (t53363, t53389, t53391, t53393, t53401)
}
