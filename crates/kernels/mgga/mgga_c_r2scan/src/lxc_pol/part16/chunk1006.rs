//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1006/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1006<F: Float>(t1058: F, t12463: F, t2207: F, t3198: F, t3290: F, t11744: F, t3591: F, t10748: F, t3187: F, t10742: F, t10759: F, t11681: F, t12120: F, t12121: F, t12132: F, t12446: F, t12450: F, t12453: F, t12457: F, t12461: F) -> F {
    let t12465 = t2207 * t1058 * t12463;
    let t12468 = t3290 * t3198;
    let t12470 = t11744 * t3591;
    let t12472 = t10748 * t3187;
    let t12474 = -t12120 - t12121 + F::cast_from(0.43663693315433241792e-2_f64) * t12446 + F::cast_from(0.21831846657716620896e-2_f64) * t12450 + F::cast_from(0.13099107994629972538e-1_f64) * t12453 - F::cast_from(0.43663693315433241792e-2_f64) * t12457 - F::cast_from(0.26198215989259945075e-1_f64) * t12461 + F::cast_from(0.65495539973149862688e-2_f64) * t12465 - t10742 + t10759 - F::cast_from(0.47609969197673950972e-2_f64) * t11681 + t12132 - F::cast_from(0.54878743191129263322e-1_f64) * t12468 - F::cast_from(0.86682217400542685632e-1_f64) * t12470 + F::cast_from(0.16463622957338778997e0_f64) * t12472;
    t12474
}
