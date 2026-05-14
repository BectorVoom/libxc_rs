//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1092/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1092<F: Float>(t15006: F, t15009: F, t15014: F, t15047: F, t15052: F, t15056: F, t15062: F, t15064: F, t4397: F, t4403: F, t6459: F, t6477: F, t6486: F, t4497: F, t6505: F, t6204: F) -> (F, F) {
    let t21930 = -0.35981577432354634426e-1 * t4397 * t6486 + 0.23987718288236422952e-1 * t4397 * t6477 - 0.17990788716177317213e-1 * t6459 * t4403 - 0.39979530480394038252e-2 * t15006 - 0.59969295720591057378e-2 * t15009 - 0.15991812192157615301e-1 * t15014 + 0.87954967056866884154e-1 * t15047 + t15052 + 0.89953943580886586067e-2 * t15056 - 0.47975436576472845902e-1 * t15062 + 0.31983624384315230602e-1 * t15064;
    let t21931 = t6505 * t4497;
    let t21932 = t6204 * t21931;
    (t21930, t21932)
}
