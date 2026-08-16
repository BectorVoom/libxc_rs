//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2222/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2222(t112: f64, t46116: f64, t1268: f64, t12725: f64, t12734: f64, t12739: f64, t12813: f64, t1458: f64, t19456: f64, t2314: f64, t2363: f64, t39235: f64, t4028: f64, t4072: f64, t45590: f64, t45602: f64, t45632: f64, t45637: f64, t45782: f64, t45814: f64, t5113: f64, t671: f64, t9348: f64, t9416: f64) -> (f64, f64) {
    let t46117 = t46116 * t112;
    let t46118 = 2.0_f64 * t1268 * t45782 + 6.0_f64 * t12725 * t2363 + 12.0_f64 * t12734 * t4072 + 6.0_f64 * t12739 * t4072 + 6.0_f64 * t12813 * t2314 + 6.0_f64 * t12813 * t5113 + 2.0_f64 * t1458 * t39235 + 6.0_f64 * t1458 * t45602 + 6.0_f64 * t1458 * t45637 + 2.0_f64 * t1458 * t45814 + 6.0_f64 * t19456 * t2363 + 2.0_f64 * t4028 * t9416 + 6.0_f64 * t4072 * t9348 + 6.0_f64 * t45632 * t671 + 6.0_f64 * t45590 + t46117;
    (t46117, t46118)
}
