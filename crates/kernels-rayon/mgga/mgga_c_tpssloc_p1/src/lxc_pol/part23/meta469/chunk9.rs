//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1395/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1395(t381: f64, t76722: f64, t1058: f64, t1060: f64, t14608: f64, t14618: f64, t1615: f64, t1625: f64, t1630: f64, t18086: f64, t21594: f64, t21614: f64, t21617: f64, t21644: f64, t21650: f64, t21653: f64, t3186: f64, t3188: f64, t43503: f64, t43505: f64, t47857: f64, t5937: f64, t69924: f64, t77485: f64, t77806: f64, t77826: f64) -> (f64, f64) {
    let t77855 = t381 * t76722;
    let t77892 = 4.0_f64 * t1058 * t1060 * t1615 * t21614 + 4.0_f64 * t1058 * t1060 * t1625 * t21594 + t1058 * t1060 * t381 * t77485 + 24.0_f64 * t21617 * t3186 * t77806 + 6.0_f64 * t3186 * t3188 * t77855 - t43503 * t43505 * t77826 - 12.0_f64 * t14608 * t21653 + 24.0_f64 * t14618 * t21644 + 4.0_f64 * t1630 * t69924 + 6.0_f64 * t18086 * t5937 - 24.0_f64 * t21650 * t47857;
    (t77855, t77892)
}
