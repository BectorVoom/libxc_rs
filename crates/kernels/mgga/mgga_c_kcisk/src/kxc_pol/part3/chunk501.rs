//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 501/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk501<F: Float>(t1474: F, t4265: F, t140: F, t1477: F, t299: F, t3529: F, t41: F, t3532: F, t451: F, t3278: F, t1402: F, t442: F, t1056: F, t1471: F, t1390: F, t1472: F, t3283: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4266 = t4265 * t1474;
    let t4269 = t140 * t299 * t1477;
    let t4271 = t41 * t3529;
    let t4272 = t451 * t3532;
    let t4274 = t4271 * t4272 * t3278;
    let t4277 = t1402 * t442;
    let t4279 = t1471 * t4277 * t1056;
    let t4282 = t451 * t1390;
    let t4284 = t1471 * t4282 * t3278;
    let t4288 = t1471 * t1472 * t3283;
    (t4266, t4269, t4271, t4272, t4274, t4277, t4279, t4284, t4288)
}
