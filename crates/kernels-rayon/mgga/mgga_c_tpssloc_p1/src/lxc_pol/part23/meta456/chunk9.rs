//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1328/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1328(t5657: f64, t10110: f64, t1519: f64, t1527: f64, t1528: f64, t17052: f64, t17090: f64, t20936: f64, t21033: f64, t21050: f64, t21054: f64, t218: f64, t252: f64, t259: f64, t2718: f64, t4147: f64, t4268: f64, t5558: f64, t5631: f64, t5636: f64, t5637: f64, t5658: f64, t68322: f64, t76372: f64, t76397: f64, t855: f64) -> f64 {
    let t76516 = t5657 * t5657;
    let t76532 = -36.0_f64 * t10110 * t5636 * t5657 * t855 + 8.0_f64 * t1527 * t21033 * t2718 * t855 + 4.0_f64 * t1519 * t20936 * t259 + t218 * t259 * t76397 + t252 * t259 * t76372 + 6.0_f64 * t259 * t5558 * t5631 + 6.0_f64 * t2718 * t76516 * t855 - 4.0_f64 * t1528 * t68322 - 6.0_f64 * t17052 * t5658 + 12.0_f64 * t17090 * t5637 - 24.0_f64 * t21050 * t4268 + 24.0_f64 * t21054 * t4147 + 24.0_f64 * t21054 * t4268;
    t76532
}
