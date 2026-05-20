//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3114/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3114<F: Float>(t12078: F, t53740: F, t12047: F, t16138: F, t372: F, t16158: F, t3106: F, t12003: F, t1659: F, t11648: F, t4879: F, t1063: F, t15790: F, t3172: F) -> (F, F, F, F, F, F, F) {
    let t54801 = t12078 * t53740;
    let t54811 = t12047 * t53740;
    let t54818 = t372 * t16138;
    let t54836 = t3106 * t16158;
    let t54838 = t1659 * t12003;
    let t54841 = t4879 * t11648;
    let t54849 = t1063 * t3172 * t15790;
    (t54801, t54811, t54818, t54836, t54838, t54841, t54849)
}
