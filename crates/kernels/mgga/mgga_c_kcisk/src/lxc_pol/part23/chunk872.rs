//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 872/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk872<F: Float>(t1376: F, t3114: F, t3866: F, t970: F, t3870: F, t3875: F, t960: F, t3878: F, t3883: F, t965: F, t3886: F, t3894: F, t1384: F, t3119: F, t1399: F, t3123: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13989 = t3114 * t1376;
    let t13991 = t970 * t3866;
    let t13993 = t970 * t3870;
    let t14001 = t960 * t3875;
    let t14003 = t960 * t3878;
    let t14011 = t965 * t3883;
    let t14014 = t965 * t3886;
    let t14025 = t965 * t3894;
    let t14027 = t3119 * t1384;
    let t14029 = t3123 * t1399;
    (t13989, t13991, t13993, t14001, t14003, t14011, t14014, t14025, t14027, t14029)
}
