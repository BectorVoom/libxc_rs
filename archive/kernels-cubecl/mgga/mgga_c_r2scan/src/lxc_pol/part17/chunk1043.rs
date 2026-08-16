//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1043/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1043<F: Float>(t2530: F, t910: F, t2526: F, t920: F, t2719: F, t938: F, t3055: F, t6363: F, t3053: F, t6212: F, t3090: F, t3056: F) -> (F, F, F, F, F, F, F, F) {
    let t28320 = t910 * t2530;
    let t28325 = t2526 * t920;
    let t28335 = t2719 * t920;
    let t28390 = t938 * t2530;
    let t28404 = t3055 * t6363;
    let t29126 = t6212 * t3053;
    let t29177 = t6212 * t3090;
    let t29194 = t6212 * t3056;
    (t28320, t28325, t28335, t28390, t28404, t29126, t29177, t29194)
}
