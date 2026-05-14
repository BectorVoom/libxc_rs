//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 939/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk939<F: Float>(t560: F, t8832: F, t481: F, t28404: F, t494: F, t3071: F, t5119: F, t528: F, t3115: F, t3433: F, t3100: F, t113: F, t29222: F, t3090: F, t9235: F, t2526: F, t2841: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29270 = t8832 * t560;
    let t29274 = t8832 * t481;
    let t29279 = t28404 * t494;
    let t29283 = t3071 * t494;
    let t29418 = t5119 * t528;
    let t29451 = t3433 * t3115;
    let t29454 = t3433 * t3100;
    let t29467 = t29222 * t113;
    let t29471 = t3090 * t481;
    let t29496 = t9235 * t481;
    let t29500 = t2841 * t2526;
    (t29270, t29274, t29279, t29283, t29418, t29451, t29454, t29467, t29471, t29496, t29500)
}
