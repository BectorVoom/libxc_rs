//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1179/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1179<F: Float>(t21531: F, t5239: F, t5396: F, t58: F, t170: F, t5398: F, t597: F, t423: F, t21115: F, t21692: F, t224: F, t234: F, t5269: F, t1809: F, t5279: F, t21677: F, t21680: F) -> (F, F, F, F, F, F, F) {
    let t22523 = 0.33872559466666666665e-1 * t5239 * t21531;
    let t22524 = t5396 * t58;
    let t22527 = t597 * t170 * t5398;
    let t22528 = t22524 * t423 * t22527;
    let t22534 = 0.18216520838430511208e7 * t234 * t21692 * t224 * t5269 * t21115;
    let t22537 = 0.21053605041484726346e2 * t234 * t5279 * t1809;
    let t22542 = 0.1078736821940706181e8 * t234 * t21677 * t224 * t21680 * t21115;
    (t22523, t22524, t22527, t22528, t22534, t22537, t22542)
}
