//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 918/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk918<F: Float>(t17353: F, t6764: F, t5013: F, t11003: F, t662: F, t6759: F, t5002: F, t7208: F, t5814: F, t657: F, t1248: F, t6714: F, t3042: F, t673: F, t1224: F) -> (F, F, F, F, F, F) {
    let t17354 = t17353 * t6764;
    let t17356 = 0.2398771828823642295e-1 * t5013 * t17354;
    let t17357 = t11003 * t662;
    let t17358 = t17357 * t6759;
    let t17360 = 0.159918121921576153e-1 * t5013 * t17358;
    let t17362 = 0.35981577432354634426e-1 * t7208 * t5002;
    let t17373 = t5814 * t657;
    let t17375 = t1248 * t17373 * t6714;
    let t17377 = t3042 * t673;
    let t17379 = t1224 * t17377 * t6714;
    (t17356, t17357, t17360, t17362, t17375, t17379)
}
