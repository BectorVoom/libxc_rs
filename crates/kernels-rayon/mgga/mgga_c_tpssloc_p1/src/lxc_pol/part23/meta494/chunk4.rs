//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1523/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1523(t6324: f64, t1390: f64, t193: f64, t20085: f64, t39658: f64, t39660: f64, t39844: f64, t39856: f64, t40224: f64, t40228: f64, t40230: f64, t40611: f64, t5160: f64, t533: f64, t6463: f64, t80112: f64, t80113: f64, t80114: f64, t80115: f64, t80116: f64, t80489: f64, t80521: f64) -> f64 {
    let t80529 = t6324 * t6324;
    let t80534 = t193 * t533 * (t80489 + t80521) * t1390 - t39658 + t39660 + t39844 + 12.0_f64 * t5160 * t20085 * t6463 - t80112 - t80113 - t39856 - t80114 - 6.0_f64 * t193 * t533 * t80529 * t40611 + t40224 + t40228 - t40230 + t80115 - t80116;
    t80534
}
