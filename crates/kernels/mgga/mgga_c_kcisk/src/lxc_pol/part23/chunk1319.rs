//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1319/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1319<F: Float>(t31932: F, t31955: F, t15208: F, t31910: F, t9310: F, t15422: F, t3934: F, t9305: F, t9304: F, t9311: F, t2677: F, t111326: F, t31967: F, t9320: F, t9307: F, t2932: F, t31966: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t111389 = t31955 * t31932;
    let t111392 = t15208 * t9310 * t31910;
    let t111395 = t3934 * t9305 * t15422;
    let t111396 = t9304 * t111395;
    let t111398 = t9311 * t31932;
    let t111400 = t2677 * t111395;
    let t111403 = t31955 * t31910;
    let t111405 = t2677 * t111326;
    let t111407 = t31967 * t9320;
    let t111409 = t31967 * t9307;
    let t111412 = t2932 * t31966 * t9307;
    (t111389, t111392, t111396, t111398, t111400, t111403, t111405, t111407, t111409, t111412)
}
