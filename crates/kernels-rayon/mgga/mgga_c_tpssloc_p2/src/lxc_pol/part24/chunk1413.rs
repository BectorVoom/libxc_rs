//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1413/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1413(t1081: f64, t11122: f64, t1877: f64, t1915: f64, t1969: f64, t22959: f64, t23286: f64, t23290: f64, t23295: f64, t23789: f64, t23813: f64, t25013: f64, t2522: f64, t25372: f64, t3231: f64, t6666: f64, t6670: f64, t6841: f64, t6848: f64, t81483: f64, t81525: f64, t82320: f64, t83613: f64, t83617: f64, t83624: f64, t83627: f64, t83630: f64, t83645: f64, t83651: f64) -> f64 {
    let t83654 = 3.0_f64 / 2.0_f64 * t1877 * t23286 * t1081 - 9.0_f64 * t81483 * t23789 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t83613 + 3.0_f64 * t1877 * t23295 * t83617 - 3.0_f64 / 2.0_f64 * t1877 * t23290 * t23813 - 9.0_f64 * t25013 * t83624 + 9.0_f64 * t25013 * t83627 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t83630 - 3.0_f64 / 2.0_f64 * t1877 * t81525 * t6848 + 9.0_f64 / 2.0_f64 * t2522 * t23286 * t6841 + t1877 * t1915 * t11122 / 2.0_f64 + 3.0_f64 * t82320 * t1969 + 3.0_f64 * t25372 * t83645 + 3.0_f64 / 2.0_f64 * t1877 * t6666 * t3231 - 9.0_f64 / 2.0_f64 * t22959 * t83651;
    t83654
}
