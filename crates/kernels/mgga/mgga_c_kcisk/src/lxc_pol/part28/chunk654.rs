//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 654/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk654<F: Float>(t1060: F, t1919: F, t7389: F, t673: F, t6941: F, t140: F, t1470: F, t3077: F, t479: F, t5242: F, t5243: F, t5246: F, t6278: F, t7368: F, t7371: F, t7375: F, t7379: F, t7383: F, t7387: F) -> (F, F, F) {
    let t7391 = t1919 * t7389 * t1060;
    let t7394 = t673 * t6941;
    let t7398 = t5242 - 0.17687407407407407407e-1 * t5243 - 0.26531111111111111111e-1 * t5246 - 0.17687407407407407407e-1 * t7368 - 0.44218518518518518518e-1 * t6278 * t7371 - 0.26531111111111111111e-1 * t1470 * t7375 + 0.53062222222222222222e-1 * t6278 * t7379 + 0.53062222222222222222e-1 * t3077 * t7383 - 0.26531111111111111111e-1 * t7387 - 0.26531111111111111111e-1 * t1470 * t7391 - 0.39796666666666666666e-1 * t140 * t479 * t7394;
    (t7391, t7394, t7398)
}
