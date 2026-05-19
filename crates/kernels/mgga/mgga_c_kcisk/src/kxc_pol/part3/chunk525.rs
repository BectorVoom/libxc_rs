//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 525/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk525<F: Float>(t1056: F, t1471: F, t4277: F, t1390: F, t451: F, t3278: F, t1472: F, t3283: F, t3904: F, t416: F, t140: F, t1429: F, t1434: F, t1460: F, t1470: F, t3560: F, t3566: F, t3588: F, t3594: F, t3620: F, t4244: F, t4253: F, t4264: F, t4266: F, t4269: F, t4274: F, t460: F, t476: F, t479: F) -> (F, F, F, F, F) {
    let t4279 = t1471 * t4277 * t1056;
    let t4282 = t451 * t1390;
    let t4284 = t1471 * t4282 * t3278;
    let t4288 = t1471 * t1472 * t3283;
    let t4291 = t416 * t3904;
    let t4295 = F::new(0.619125e-2) * t4244 * t460 + F::new(0.1857375e-1) * t1460 * t1429 - F::new(0.123825e-1) * t1460 * t1434 + F::new(0.46434375e-2) * t476 * t3560 - F::new(0.1857375e-1) * t4253 * t3566 + F::new(0.9286875e-2) * t476 * t3588 + F::new(0.123825e-1) * t476 * t3594 - F::new(0.619125e-2) * t476 * t3620 + t4264 - F::cast_from(0.35374814814814814814e-1_f64) * t4266 - F::cast_from(0.53062222222222222222e-1_f64) * t4269 - F::cast_from(0.44218518518518518518e-1_f64) * t1470 * t4274 - F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t4279 + F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t4284 - F::cast_from(0.26531111111111111111e-1_f64) * t1470 * t4288 - F::cast_from(0.39796666666666666666e-1_f64) * t140 * t479 * t4291;
    (t4279, t4284, t4288, t4291, t4295)
}
