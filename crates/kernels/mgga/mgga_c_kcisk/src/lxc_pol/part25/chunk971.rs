//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 971/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk971<F: Float>(t1769: F, t7213: F, t2456: F, t4995: F, t2449: F, t2464: F, t4597: F, t3290: F, t10802: F, t3293: F, t7256: F, t5015: F, t695: F, t7268: F, t1060: F, t2364: F, t5038: F) -> (F, F, F, F, F, F, F) {
    let t17218 = t7213 * t1769;
    let t17220 = t2456 * t4995;
    let t17222 = t2449 * t4995;
    let t17224 = t2464 * t4597;
    let t17225 = t17224 * t3290;
    let t17226 = t10802 * t17225;
    let t17229 = t7256 * t3293;
    let t17230 = t5015 * t17229;
    let t17233 = t7268 * t695;
    let t17234 = t17233 * t1060;
    let t17235 = t5015 * t17234;
    let t17240 = t2364 * t5038;
    (t17218, t17220, t17222, t17226, t17230, t17235, t17240)
}
