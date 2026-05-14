//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1253/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1253<F: Float>(t2545: F, t6194: F, t920: F, t24997: F, t2530: F, t910: F, t2526: F, t2719: F, t938: F, t3055: F, t6363: F, t113: F, t1398: F, t3158: F, t5: F, t378: F, t9029: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28300 = t2545 * t920 * t6194;
    let t28301 = t24997 * t28300;
    let t28320 = t910 * t2530;
    let t28325 = t2526 * t920;
    let t28335 = t2719 * t920;
    let t28390 = t938 * t2530;
    let t28404 = t3055 * t6363;
    let t28418 = t113 * t2526;
    let t28460 = t5 * t1398 * t3158;
    let t28463 = t5 * t378 * t9029;
    (t28300, t28301, t28320, t28325, t28335, t28390, t28404, t28418, t28460, t28463)
}
