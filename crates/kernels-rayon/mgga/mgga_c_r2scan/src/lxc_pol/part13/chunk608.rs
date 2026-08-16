//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 608/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk608(t254: f64, t3344: f64, t3283: f64, t3286: f64, t3289: f64, t3291: f64, t3297: f64, t3301: f64, t3306: f64, t3310: f64, t3312: f64, t3318: f64, t3323: f64, t3326: f64, t3330: f64, t3334: f64, t3338: f64) -> (f64, f64) {
    let t3345 = t254 * t3344;
    let t3346 = 0.23804984598836975486e-2_f64 * t3345;
    let t3347 = -t3283 + t3286 - t3289 - 0.54878743191129263322e-1_f64 * t3291 - 0.27439371595564631661e-1_f64 * t3297 - 0.43341108700271342816e-1_f64 * t3301 - 0.13002332610081402845e0_f64 * t3306 - 0.43341108700271342816e-1_f64 * t3310 + 0.43341108700271342816e-1_f64 * t3312 - t3318 + t3323 + 0.21831846657716620896e-2_f64 * t3326 + 0.65495539973149862688e-2_f64 * t3330 + 0.21831846657716620896e-2_f64 * t3334 - 0.21831846657716620896e-2_f64 * t3338 - t3346;
    (t3346, t3347)
}
