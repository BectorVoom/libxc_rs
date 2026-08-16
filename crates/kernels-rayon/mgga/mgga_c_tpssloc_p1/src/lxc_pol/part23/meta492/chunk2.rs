//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1511/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1511(t550: f64, t80150: f64, t1336: f64, t1380: f64, t19654: f64, t19739: f64, t19743: f64, t19810: f64, t19815: f64, t20473: f64, t20554: f64, t20568: f64, t20632: f64, t20638: f64, t20643: f64, t20645: f64, t3897: f64, t5234: f64, t5334: f64, t5344: f64, t5348: f64, t6415: f64, t6454: f64, t80085: f64) -> (f64, f64) {
    let t80151 = t80150 * t550;
    let t80164 = -t1336 * t1380 * t80151 - 4.0_f64 * t1336 * t20554 * t5348 - 4.0_f64 * t1336 * t20568 * t5348 + 6.0_f64 * t1336 * t3897 * t80085 + 24.0_f64 * t19739 * t20473 * t5334 - 6.0_f64 * t19743 * t5344 * t6415 + 24.0_f64 * t19654 * t20638 - 12.0_f64 * t19810 * t20632 - 6.0_f64 * t19815 * t6454 - 4.0_f64 * t20643 * t5234 - 12.0_f64 * t20645 * t5234;
    (t80151, t80164)
}
