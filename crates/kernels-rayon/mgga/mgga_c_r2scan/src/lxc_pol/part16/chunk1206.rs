//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1206/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1206(t29779: f64, t3332: f64, t7614: f64, t29270: f64, t6165: f64, t10856: f64, t9273: f64, t39887: f64, t39895: f64, t43376: f64, t43379: f64, t43381: f64, t43384: f64, t43387: f64, t43390: f64, t43393: f64) -> f64 {
    let t43396 = t7614 * t3332 * t29779;
    let t43399 = t6165 * t3332 * t29270;
    let t43401 = t10856 * t9273;
    let t43403 = -t39887 + 0.13099107994629972538e-1_f64 * t43376 - 0.87327386630866483584e-2_f64 * t43379 - 0.13099107994629972538e-1_f64 * t43381 - 0.13099107994629972538e-1_f64 * t43384 - 0.13099107994629972538e-1_f64 * t43387 + 0.26198215989259945075e-1_f64 * t43390 + 0.13099107994629972538e0_f64 * t43393 - 0.5239643197851989015e-1_f64 * t43396 - 0.65495539973149862688e-2_f64 * t43399 - 0.97574405393827830187e-2_f64 * t43401 - t39895;
    t43403
}
