//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1202/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1202<F: Float>(t10760: F, t22868: F, t29471: F, t2196: F, t29779: F, t3308: F, t10710: F, t30119: F, t37586: F, t3602: F, t37755: F, t7605: F) -> (F, F, F, F) {
    let t43348 = t22868 * t10760 * t29471;
    let t43351 = t2196 * t3308 * t29779;
    let t43356 = t37586 * t10710 * t30119;
    let t43359 = t37755 * t3602 * t7605;
    (t43348, t43351, t43356, t43359)
}
