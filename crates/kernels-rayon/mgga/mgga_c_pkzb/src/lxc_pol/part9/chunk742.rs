//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 742/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk742(t1542: f64, t546: f64, t1548: f64, t513: f64, t1816: f64, t639: f64, t135: f64, t144: f64, t1535: f64, t1536: f64, t1692: f64, t192: f64, t5011: f64, t5019: f64, t5022: f64, t5025: f64, t5162: f64, t5165: f64, t5171: f64, t5176: f64, t5178: f64, t5180: f64, t5181: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5186 = 60.0_f64 * t1542 * t546;
    let t5187 = t1548 * t513;
    let t5188 = 96.0_f64 * t5187;
    let t5189 = t1542 * t513;
    let t5190 = 60.0_f64 * t5189;
    let t5191 = t1816 * t639;
    let t5195 = 2.0_f64 * t135 * t144 * t5162 * t5165 + 6.0_f64 * t135 * t192 * t5181 + 9.0_f64 * t1535 * t1536 * t1692 + 9.0_f64 * t1535 * t5191 * t568 - t5011 + t5019 - t5022 + t5025 + t5171 + t5176 + t5178 + t5180 + t5186 - t5188 + t5190;
    (t5186, t5188, t5189, t5190, t5191, t5195)
}
