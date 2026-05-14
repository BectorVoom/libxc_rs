//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 887/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk887<F: Float>(t15929: F, t15930: F, t5192: F, t15928: F, t1755: F, t2454: F, t4972: F, t1800: F, t1869: F, t4817: F, t7070: F, t5204: F, t6719: F, t1801: F, t695: F, t6713: F) -> (F, F, F, F, F, F, F, F) {
    let t15931 = t15929 * t15930;
    let t15932 = t5192 * t15931;
    let t15933 = t15928 * t15932;
    let t15936 = t2454 * t1755;
    let t15937 = t15936 * t4972;
    let t15938 = t1800 * t15937;
    let t15939 = t1869 * t15938;
    let t15941 = t4817 * t7070;
    let t15942 = t1869 * t15941;
    let t15944 = t6719 * t5204;
    let t15945 = t1869 * t15944;
    let t15947 = t1801 * t695;
    let t15948 = t1800 * t15947;
    let t15949 = t6713 * t15948;
    (t15931, t15933, t15937, t15939, t15942, t15945, t15947, t15949)
}
