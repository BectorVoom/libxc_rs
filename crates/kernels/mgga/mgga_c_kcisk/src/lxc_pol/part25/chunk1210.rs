//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1210/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1210<F: Float>(t2642: F, t5515: F, t2455: F, t397: F, t31861: F, t31863: F, t31865: F, t31875: F, t31884: F, t31998: F, t31999: F, t1053: F, t3186: F, t31860: F, t3181: F, t32581: F, t43151: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t74798 = t2642 * t5515;
    let t74846 = t2455 * t397;
    let t109134 = 3.0 * t31861;
    let t109135 = 12.0 * t31863;
    let t109136 = 6.0 * t31865;
    let t109141 = 12.0 * t31875;
    let t109144 = 6.0 * t31884;
    let t109148 = 3.0 * t31998;
    let t109149 = 3.0 * t31999;
    let t109152 = 6.0 * t3186 * t31860 * t1053;
    let t109154 = 3.0 * t3181 * t31860;
    let t109160 = 18.0 * t43151 * t32581;
    (t74798, t74846, t109134, t109135, t109136, t109141, t109144, t109148, t109149, t109152, t109154, t109160)
}
