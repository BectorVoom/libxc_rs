//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 821/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk821<F: Float>(t481: F, t494: F, t7338: F, t7337: F, t560: F, t5109: F, t1593: F, t921: F, t2533: F, t2551: F, t7321: F, t2294: F, t2568: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7339 = t494 * t481;
    let t7340 = t7338 * t7339;
    let t7341 = t7337 * t7340;
    let t7344 = t494 * t560;
    let t7345 = t7338 * t7344;
    let t7346 = t5109 * t7345;
    let t7349 = t5109 * t7340;
    let t7352 = t921 * t1593;
    let t7353 = t5109 * t7352;
    let t7356 = t2533 * t2551;
    let t7357 = t7321 * t7356;
    let t7360 = t2294 * t2568;
    (t7340, t7341, t7345, t7346, t7349, t7352, t7353, t7356, t7357, t7360)
}
