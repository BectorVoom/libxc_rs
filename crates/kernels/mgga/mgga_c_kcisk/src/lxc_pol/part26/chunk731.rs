//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 731/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk731<F: Float>(t1140: F, t1147: F, t2705: F, t289: F, t3437: F, t3442: F, t9336: F, t9338: F, t9339: F, t9342: F, t9359: F, t9390: F, t9392: F, t9395: F, t9404: F, t1151: F, t295: F) -> (F, F) {
    let t9406 = -t1140 * t9404 - t1147 * t9392 - t2705 * t3437 + t289 * t9390 + 2.0 * t3442 * t9395 - t9336 + t9338 + t9339 - t9342 + t9359;
    let t9408 = t1151 * t295;
    (t9406, t9408)
}
