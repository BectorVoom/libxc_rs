//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 623/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk623(t3275: f64, t3277: f64, t3472: f64, t3282: f64, t3285: f64, t3288: f64, t3317: f64, t3322: f64, t3345: f64, t3291: f64, t3297: f64, t3301: f64, t3306: f64, t3310: f64, t3312: f64, t3326: f64, t3330: f64, t3334: f64, t3338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3474 = t3275 * t3472 * t3277;
    let t3475 = 5.0_f64 / 16.0_f64 * t3474;
    let t3476 = 0.54878743191129263322e-2_f64 * t3282;
    let t3477 = 0.97574405393827830187e-2_f64 * t3285;
    let t3478 = 0.12805040077930161442e0_f64 * t3288;
    let t3485 = 0.23115257973478049502e0_f64 * t3317;
    let t3486 = 0.46574606203128791246e-1_f64 * t3322;
    let t3491 = 0.47609969197673950973e-2_f64 * t3345;
    let t3492 = -t3476 + t3477 - t3478 - 0.10975748638225852664e0_f64 * t3291 - 0.54878743191129263322e-1_f64 * t3297 - 0.86682217400542685632e-1_f64 * t3301 - 0.2600466522016280569e0_f64 * t3306 - 0.86682217400542685632e-1_f64 * t3310 + 0.86682217400542685632e-1_f64 * t3312 - t3485 + t3486 + 0.43663693315433241794e-2_f64 * t3326 + 0.13099107994629972538e-1_f64 * t3330 + 0.43663693315433241794e-2_f64 * t3334 - 0.43663693315433241794e-2_f64 * t3338 - t3491;
    (t3475, t3476, t3477, t3478, t3485, t3486, t3491, t3492)
}
