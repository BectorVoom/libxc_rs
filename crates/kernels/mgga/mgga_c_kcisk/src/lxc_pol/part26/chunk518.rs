//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 518/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk518<F: Float>(t3857: F, t1390: F, t967: F, t1056: F, t1399: F, t970: F, t1376: F, t960: F, t1384: F, t965: F, t167: F, t3532: F, t408: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3858 = 0.15538616723388920628e-3 * t3857;
    let t3859 = t967 * t1390;
    let t3860 = t3859 * t1056;
    let t3864 = t970 * t1399;
    let t3873 = t960 * t1376;
    let t3881 = t965 * t1384;
    let t3891 = t167 * t3532;
    let t3923 = t408 * t408;
    let t3924 = 1.0 / t3923;
    (t3858, t3859, t3860, t3864, t3873, t3881, t3891, t3923, t3924)
}
