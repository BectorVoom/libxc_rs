//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1328/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1328<F: Float>(t5657: F, t10110: F, t1519: F, t1527: F, t1528: F, t17052: F, t17090: F, t20936: F, t21033: F, t21050: F, t21054: F, t218: F, t252: F, t259: F, t2718: F, t4147: F, t4268: F, t5558: F, t5631: F, t5636: F, t5637: F, t5658: F, t68322: F, t76372: F, t76397: F, t855: F) -> F {
    let t76516 = t5657 * t5657;
    let t76532 = -F::cast_from(36.0_f64) * t10110 * t5636 * t5657 * t855 + F::cast_from(8.0_f64) * t1527 * t21033 * t2718 * t855 + F::cast_from(4.0_f64) * t1519 * t20936 * t259 + t218 * t259 * t76397 + t252 * t259 * t76372 + F::cast_from(6.0_f64) * t259 * t5558 * t5631 + F::cast_from(6.0_f64) * t2718 * t76516 * t855 - F::cast_from(4.0_f64) * t1528 * t68322 - F::cast_from(6.0_f64) * t17052 * t5658 + F::cast_from(12.0_f64) * t17090 * t5637 - F::cast_from(24.0_f64) * t21050 * t4268 + F::cast_from(24.0_f64) * t21054 * t4147 + F::cast_from(24.0_f64) * t21054 * t4268;
    t76532
}
