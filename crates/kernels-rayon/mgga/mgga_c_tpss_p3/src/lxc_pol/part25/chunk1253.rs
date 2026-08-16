//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1253/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1253(t1378: f64, t226: f64, t6337: f64, t5577: f64, t1805: f64, t4758: f64, t21638: f64, t1708: f64, t21608: f64, t228: f64, t1396: f64, t1707: f64, t18006: f64, t1809: f64, t19736: f64, t20449: f64, t21299: f64, t21609: f64, t21624: f64, t21627: f64, t21631: f64, t21635: f64, t21640: f64, t253: f64, t4784: f64, t4800: f64, t5571: f64, t5834: f64, t6135: f64, t6343: f64, t6348: f64, t6351: f64) -> (f64, f64, f64, f64, f64) {
    let t21644 = t6337 * t1378 * t226;
    let t21645 = t5577 * t21644;
    let t21650 = t5577 * t1805 * t4758 * t226;
    let t21653 = t5577 * t21638 * t226;
    let t21656 = t1708 * t228 * t21608;
    let t21658 = -2.0_f64 * t1396 * t20449 - t1707 * t21656 - 4.0_f64 * t18006 * t21627 - t1809 * t21299 + 4.0_f64 * t19736 * t6343 + 2.0_f64 * t19736 * t6348 + t21609 * t253 - 6.0_f64 * t21624 * t5571 + 4.0_f64 * t21631 * t5571 + 2.0_f64 * t21635 * t5571 - 2.0_f64 * t21640 * t5571 + 2.0_f64 * t21645 * t5571 + t21650 * t5571 + t21653 * t5571 + 2.0_f64 * t4784 * t5834 - t4800 * t5834 - 2.0_f64 * t6135 * t6351;
    (t21645, t21650, t21653, t21656, t21658)
}
