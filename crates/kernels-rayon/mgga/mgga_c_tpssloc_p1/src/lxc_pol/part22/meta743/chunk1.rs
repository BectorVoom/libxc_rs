//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2465/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2465(t17607: f64, t4571: f64, t1011: f64, t1019: f64, t69923: f64, t1025: f64, t1622: f64, t21405: f64, t21580: f64, t21609: f64, t3048: f64, t3117: f64, t43211: f64, t61659: f64, t61663: f64, t61665: f64, t61710: f64, t70132: f64) -> f64 {
    let t70138 = t17607 * t4571;
    let t70148 = t69923 * t1011 * t1019;
    let t70151 = t3117 * t21609 / 768.0_f64 - t70132 / 1152.0_f64 + 5.0_f64 / 432.0_f64 * t3048 * t21580 - t61710 * t1622 / 288.0_f64 + t70138 / 2304.0_f64 - t3048 * t21609 / 144.0_f64 + t61659 / 1152.0_f64 - t61663 / 2304.0_f64 + t61665 / 1536.0_f64 - t43211 * t21405 / 576.0_f64 + t70148 * t1025 / 3072.0_f64;
    t70151
}
