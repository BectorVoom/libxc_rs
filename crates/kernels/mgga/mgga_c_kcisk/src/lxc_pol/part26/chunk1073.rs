//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1073/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1073<F: Float>(t9301: F, t9320: F, t9307: F, t15217: F, t2676: F, t2932: F, t9310: F, t31932: F, t9304: F, t15208: F, t31910: F, t140: F, t190: F, t3128: F, t119: F, t3032: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31948 = t9301 * t9320;
    let t31950 = t9301 * t9307;
    let t31952 = t15217 * t2676;
    let t31953 = t31952 * t9307;
    let t31955 = t2932 * t9310;
    let t31956 = t31955 * t9307;
    let t31958 = t9304 * t31932;
    let t31960 = t15208 * t2676;
    let t31961 = t31960 * t31910;
    let t31964 = t140 * t3128 * t190;
    let t31966 = t3032 * t119;
    (t31948, t31950, t31952, t31953, t31955, t31956, t31958, t31960, t31961, t31964, t31966)
}
