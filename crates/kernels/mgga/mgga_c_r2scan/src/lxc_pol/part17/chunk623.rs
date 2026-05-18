//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 623/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk623<F: Float>(t3275: F, t3277: F, t3472: F, t3282: F, t3285: F, t3288: F, t3317: F, t3322: F, t3345: F, t3291: F, t3297: F, t3301: F, t3306: F, t3310: F, t3312: F, t3326: F, t3330: F, t3334: F, t3338: F) -> (F, F, F, F, F, F, F, F) {
    let t3474 = t3275 * t3472 * t3277;
    let t3475 = F::new(5.0) / F::new(16.0) * t3474;
    let t3476 = F::new(0.54878743191129263322e-2) * t3282;
    let t3477 = F::new(0.97574405393827830187e-2) * t3285;
    let t3478 = F::new(0.12805040077930161442e0) * t3288;
    let t3485 = F::new(0.23115257973478049502e0) * t3317;
    let t3486 = F::new(0.46574606203128791246e-1) * t3322;
    let t3491 = F::new(0.47609969197673950973e-2) * t3345;
    let t3492 = -t3476 + t3477 - t3478 - F::new(0.10975748638225852664e0) * t3291 - F::new(0.54878743191129263322e-1) * t3297 - F::new(0.86682217400542685632e-1) * t3301 - F::new(0.2600466522016280569e0) * t3306 - F::new(0.86682217400542685632e-1) * t3310 + F::new(0.86682217400542685632e-1) * t3312 - t3485 + t3486 + F::new(0.43663693315433241794e-2) * t3326 + F::new(0.13099107994629972538e-1) * t3330 + F::new(0.43663693315433241794e-2) * t3334 - F::new(0.43663693315433241794e-2) * t3338 - t3491;
    (t3475, t3476, t3477, t3478, t3485, t3486, t3491, t3492)
}
