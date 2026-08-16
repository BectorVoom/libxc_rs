//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1212/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1212(t9623: f64, t9638: f64, t10007: f64, t10009: f64, t13350: f64, t210: f64, t2553: f64, t2571: f64, t2605: f64, t2643: f64, t2645: f64, t2646: f64, t2684: f64, t2707: f64, t40998: f64, t41009: f64, t41012: f64, t41014: f64, t41025: f64, t4178: f64, t4180: f64, t804: f64, t829: f64, t9516: f64, t9559: f64, t9616: f64, t9621: f64, t9626: f64, t9642: f64, t9990: f64) -> f64 {
    let t41031 = t9638 * t9623;
    let t41037 = -t9990 * t2707 / 128.0_f64 - 7.0_f64 / 4.0_f64 * t40998 - 3.0_f64 / 2.0_f64 * t9559 * t210 * t2605 * t2553 + t2571 * t210 * t804 * t9516 / 4.0_f64 + 35.0_f64 / 12.0_f64 * t41009 + 7.0_f64 / 3.0_f64 * t41012 + t4178 * t4180 * t2646 * t41014 / 384.0_f64 + t9642 * t10009 / 64.0_f64 + t2643 * t2645 * t9626 * t10007 / 128.0_f64 - 7.0_f64 / 96.0_f64 * t41025 - t2643 * t4180 * t9621 * t2684 / 512.0_f64 + 7.0_f64 / 384.0_f64 * t41031 - 5.0_f64 / 64.0_f64 * t2643 * t13350 * t829 * t9616;
    t41037
}
