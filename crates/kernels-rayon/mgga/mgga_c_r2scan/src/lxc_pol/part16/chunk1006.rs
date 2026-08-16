//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1006/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1006(t1058: f64, t12463: f64, t2207: f64, t3198: f64, t3290: f64, t11744: f64, t3591: f64, t10748: f64, t3187: f64, t10742: f64, t10759: f64, t11681: f64, t12120: f64, t12121: f64, t12132: f64, t12446: f64, t12450: f64, t12453: f64, t12457: f64, t12461: f64) -> f64 {
    let t12465 = t2207 * t1058 * t12463;
    let t12468 = t3290 * t3198;
    let t12470 = t11744 * t3591;
    let t12472 = t10748 * t3187;
    let t12474 = -t12120 - t12121 + 0.43663693315433241792e-2_f64 * t12446 + 0.21831846657716620896e-2_f64 * t12450 + 0.13099107994629972538e-1_f64 * t12453 - 0.43663693315433241792e-2_f64 * t12457 - 0.26198215989259945075e-1_f64 * t12461 + 0.65495539973149862688e-2_f64 * t12465 - t10742 + t10759 - 0.47609969197673950972e-2_f64 * t11681 + t12132 - 0.54878743191129263322e-1_f64 * t12468 - 0.86682217400542685632e-1_f64 * t12470 + 0.16463622957338778997e0_f64 * t12472;
    t12474
}
