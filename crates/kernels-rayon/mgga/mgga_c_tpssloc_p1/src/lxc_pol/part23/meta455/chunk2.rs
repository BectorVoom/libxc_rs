//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1315/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1315(t232: f64, t76073: f64, t5584: f64, t40933: f64, t9975: f64, t13251: f64, t13262: f64, t1484: f64, t16839: f64, t16891: f64, t20885: f64, t20887: f64, t20972: f64, t2632: f64, t2643: f64, t2645: f64, t4178: f64, t4180: f64, t5527: f64, t5591: f64, t5617: f64, t67607: f64, t67612: f64, t67625: f64, t67637: f64, t67639: f64, t68246: f64, t9646: f64) -> (f64, f64, f64, f64, f64) {
    let t76074 = t76073 * t232;
    let t76085 = t5584 * t5584;
    let t76086 = t76085 * t40933;
    let t76090 = t76085 * t9975;
    let t76132 = t2643 * t2645 * t67607 * t5591 / 192.0_f64 - 7.0_f64 / 48.0_f64 * t67612 + 7.0_f64 / 48.0_f64 * t67625 - 5.0_f64 / 128.0_f64 * t2643 * t9646 * t16839 * t20972 + t13262 * t2645 * t67607 * t9975 * t1484 / 32.0_f64 - 3.0_f64 / 256.0_f64 * t13262 * t4180 * t16839 * t68246 + 5.0_f64 / 64.0_f64 * t4178 * t9646 * t16839 * t2632 * t5527 + 35.0_f64 / 96.0_f64 * t67637 + 7.0_f64 / 384.0_f64 * t67639 + t2643 * t2645 * t16891 * t20885 / 128.0_f64 + t13251 * t20887 / 64.0_f64 - t2643 * t4180 * t16891 * t5617 / 512.0_f64;
    (t76074, t76085, t76086, t76090, t76132)
}
