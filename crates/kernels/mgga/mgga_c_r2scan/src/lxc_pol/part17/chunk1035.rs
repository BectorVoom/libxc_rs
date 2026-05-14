//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1035/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1035<F: Float>(t3594: F, t39745: F, t10760: F, t2147: F, t28005: F, t11727: F, t11748: F, t22790: F, t31064: F, t22868: F, t29471: F, t2196: F, t29779: F, t3308: F, t10710: F, t30119: F, t37586: F) -> (F, F, F, F, F, F, F) {
    let t43332 = t39745 * t3594;
    let t43335 = t2147 * t10760 * t28005;
    let t43337 = t11748 * t11727;
    let t43340 = t22790 * t10760 * t31064;
    let t43348 = t22868 * t10760 * t29471;
    let t43351 = t2196 * t3308 * t29779;
    let t43356 = t37586 * t10710 * t30119;
    (t43332, t43335, t43337, t43340, t43348, t43351, t43356)
}
