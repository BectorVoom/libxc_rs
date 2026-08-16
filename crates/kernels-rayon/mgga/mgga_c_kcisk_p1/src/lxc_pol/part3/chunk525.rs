//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 525/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk525(t1056: f64, t1471: f64, t4277: f64, t1390: f64, t451: f64, t3278: f64, t1472: f64, t3283: f64, t3904: f64, t416: f64, t140: f64, t1429: f64, t1434: f64, t1460: f64, t1470: f64, t3560: f64, t3566: f64, t3588: f64, t3594: f64, t3620: f64, t4244: f64, t4253: f64, t4264: f64, t4266: f64, t4269: f64, t4274: f64, t460: f64, t476: f64, t479: f64) -> (f64, f64, f64, f64, f64) {
    let t4279 = t1471 * t4277 * t1056;
    let t4282 = t451 * t1390;
    let t4284 = t1471 * t4282 * t3278;
    let t4288 = t1471 * t1472 * t3283;
    let t4291 = t416 * t3904;
    let t4295 = 0.619125e-2_f64 * t4244 * t460 + 0.1857375e-1_f64 * t1460 * t1429 - 0.123825e-1_f64 * t1460 * t1434 + 0.46434375e-2_f64 * t476 * t3560 - 0.1857375e-1_f64 * t4253 * t3566 + 0.9286875e-2_f64 * t476 * t3588 + 0.123825e-1_f64 * t476 * t3594 - 0.619125e-2_f64 * t476 * t3620 + t4264 - 0.35374814814814814814e-1_f64 * t4266 - 0.53062222222222222222e-1_f64 * t4269 - 0.44218518518518518518e-1_f64 * t1470 * t4274 - 0.53062222222222222222e-1_f64 * t1470 * t4279 + 0.53062222222222222222e-1_f64 * t1470 * t4284 - 0.26531111111111111111e-1_f64 * t1470 * t4288 - 0.39796666666666666666e-1_f64 * t140 * t479 * t4291;
    (t4279, t4284, t4288, t4291, t4295)
}
