//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1089/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1089(t10943: f64, t11603: f64, t2333: f64, t2847: f64, t2850: f64, t4176: f64, t3270: f64, t3348: f64, t910: f64, t3618: f64, t792: f64, t11002: f64) -> (f64, f64, f64, f64, f64) {
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
