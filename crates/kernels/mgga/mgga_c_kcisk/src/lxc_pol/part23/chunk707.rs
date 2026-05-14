//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 707/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk707<F: Float>(t2209: F, t442: F, t1056: F, t1471: F, t416: F, t5864: F, t140: F, t1470: F, t3077: F, t4264: F, t4266: F, t4269: F, t479: F, t6275: F, t6278: F, t6280: F, t6284: F, t6288: F, t6292: F, t6296: F) -> (F, F, F, F) {
    let t6298 = t2209 * t442;
    let t6300 = t1471 * t6298 * t1056;
    let t6303 = t416 * t5864;
    let t6307 = t4264 - 0.17687407407407407407e-1 * t4266 - 0.26531111111111111111e-1 * t4269 - 0.17687407407407407407e-1 * t6275 - 0.44218518518518518518e-1 * t6278 * t6280 - 0.26531111111111111111e-1 * t1470 * t6284 + 0.53062222222222222222e-1 * t6278 * t6288 - 0.53062222222222222222e-1 * t3077 * t6292 - 0.26531111111111111111e-1 * t6296 - 0.26531111111111111111e-1 * t1470 * t6300 - 0.39796666666666666666e-1 * t140 * t479 * t6303;
    (t6298, t6300, t6303, t6307)
}
