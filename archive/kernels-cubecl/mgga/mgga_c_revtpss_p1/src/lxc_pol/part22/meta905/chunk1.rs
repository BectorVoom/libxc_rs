//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3104/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3104<F: Float>(t11933: F, t16035: F, t11774: F, t127: F, t15585: F, t4872: F, t16226: F, t16229: F, t53405: F, t3230: F, t4857: F, t11817: F, t4858: F) -> (F, F, F, F, F) {
    let t54324 = t11933 * t16035;
    let t54341 = t11774 * t127 * t4872 * t15585;
    let t54348 = t16226 * t53405 * t16229;
    let t54384 = t4857 * t3230;
    let t54387 = t4858 * t11817;
    (t54324, t54341, t54348, t54384, t54387)
}
