//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 608/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk608<F: Float>(t254: F, t3344: F, t3283: F, t3286: F, t3289: F, t3291: F, t3297: F, t3301: F, t3306: F, t3310: F, t3312: F, t3318: F, t3323: F, t3326: F, t3330: F, t3334: F, t3338: F) -> (F, F) {
    let t3345 = t254 * t3344;
    let t3346 = F::cast_from(0.23804984598836975486e-2_f64) * t3345;
    let t3347 = -t3283 + t3286 - t3289 - F::cast_from(0.54878743191129263322e-1_f64) * t3291 - F::cast_from(0.27439371595564631661e-1_f64) * t3297 - F::cast_from(0.43341108700271342816e-1_f64) * t3301 - F::cast_from(0.13002332610081402845e0_f64) * t3306 - F::cast_from(0.43341108700271342816e-1_f64) * t3310 + F::cast_from(0.43341108700271342816e-1_f64) * t3312 - t3318 + t3323 + F::cast_from(0.21831846657716620896e-2_f64) * t3326 + F::cast_from(0.65495539973149862688e-2_f64) * t3330 + F::cast_from(0.21831846657716620896e-2_f64) * t3334 - F::cast_from(0.21831846657716620896e-2_f64) * t3338 - t3346;
    (t3346, t3347)
}
