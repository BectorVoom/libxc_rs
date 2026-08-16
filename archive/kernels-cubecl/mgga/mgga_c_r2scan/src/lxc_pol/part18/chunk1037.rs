//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1037/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1037<F: Float>(t3090: F, t6212: F, t3056: F, t2892: F, t494: F, t560: F, t8832: F, t481: F, t28404: F, t3071: F, t5119: F, t528: F) -> (F, F, F, F, F, F, F, F) {
    let t29177 = t6212 * t3090;
    let t29194 = t6212 * t3056;
    let t29222 = t2892 * t494;
    let t29270 = t8832 * t560;
    let t29274 = t8832 * t481;
    let t29279 = t28404 * t494;
    let t29283 = t3071 * t494;
    let t29418 = t5119 * t528;
    (t29177, t29194, t29222, t29270, t29274, t29279, t29283, t29418)
}
