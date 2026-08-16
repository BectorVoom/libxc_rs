//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 804/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk804(t13839: f64, t5171: f64, t21752: f64, t9791: f64, t2606: f64, t21672: f64, t3885: f64, t2599: f64, t14233: f64, t18633: f64, t18746: f64, t1901: f64, t193: f64, t21719: f64, t21724: f64, t21728: f64, t21732: f64, t21736: f64, t21740: f64, t21744: f64, t21748: f64, t21754: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21757 = t13839 * t5171;
    let t21760 = t9791 * t21752;
    let t21761 = t2606 * t21760;
    let t21764 = t3885 * t21672;
    let t21765 = t2599 * t21764;
    let t21768 = -4.0_f64 / 9.0_f64 * t14233 + t89 * t193 * t21719 / 3.0_f64 - t446 * t21724 / 9.0_f64 - 10.0_f64 / 81.0_f64 * t446 * t21728 - t446 * t21732 / 3.0_f64 - t446 * t21736 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t446 * t21740 - 2.0_f64 / 3.0_f64 * t18633 + 2.0_f64 / 3.0_f64 * t1901 * t21744 - 2.0_f64 / 3.0_f64 * t1901 * t21748 - t18746 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t21754 + 2.0_f64 / 3.0_f64 * t1901 * t21757 - 2.0_f64 / 3.0_f64 * t1901 * t21761 - 2.0_f64 / 3.0_f64 * t1901 * t21765;
    (t21757, t21760, t21761, t21764, t21765, t21768)
}
