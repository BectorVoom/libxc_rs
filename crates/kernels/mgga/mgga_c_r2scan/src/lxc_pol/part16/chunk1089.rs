//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1089/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1089<F: Float>(t10943: F, t11603: F, t2333: F, t2847: F, t2850: F, t4176: F, t3270: F, t3348: F, t910: F, t3618: F, t792: F, t11002: F) -> (F, F, F, F, F) {
    let t39290 = t10943 * t11603;
    let t39299 = t2333 * t2847;
    let t39311 = t4176 * t2850;
    let t39312 = t3270 * t39311;
    let t39323 = t3348 * t910;
    let t39324 = t3270 * t39323;
    let t39331 = t3618 * t792;
    let t39332 = t11002 * t39331;
    (t39290, t39299, t39312, t39324, t39332)
}
