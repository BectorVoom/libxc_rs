//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 767/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk767<F: Float>(t2046: F, t3801: F, t4170: F, t4160: F, t2011: F, t4136: F, t12322: F, t1395: F, t1464: F, t3728: F, t5882: F, t2001: F, t3954: F, t1396: F, t4123: F, t5678: F) -> (F, F, F, F, F, F, F, F) {
    let t15925 = t2046 * t3801;
    let t15926 = t4170 * t15925;
    let t15927 = t4160 * t15926;
    let t15929 = t2011 * t4136;
    let t15930 = t12322 * t15929;
    let t15931 = t1395 * t15930;
    let t15932 = t1464 * t15931;
    let t15934 = t3728 * t5882;
    let t15936 = t2001 * t3954;
    let t15937 = t1396 * t15936;
    let t15938 = t4123 * t15937;
    let t15939 = t1464 * t15938;
    let t15941 = t3728 * t5678;
    (t15925, t15927, t15929, t15932, t15934, t15936, t15939, t15941)
}
