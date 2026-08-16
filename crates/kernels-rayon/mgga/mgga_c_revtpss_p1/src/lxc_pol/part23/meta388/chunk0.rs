//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1734/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1734(t16868: f64, t16712: f64, t16892: f64, t16708: f64, t1179: f64, t5155: f64, t1719: f64, t3383: f64, t1749: f64, t3520: f64, t3495: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17050 = 0.13892666666666666667e0_f64 * t16868;
    let t17052 = 0.34431666666666666666e0_f64 * t16712;
    let t17066 = 0.27785333333333333334e0_f64 * t16892;
    let t17075 = 0.22954444444444444444e0_f64 * t16708;
    let t17089 = t5155 * t1179;
    let t17092 = t1719 * t3383;
    let t17097 = t1749 * t3520;
    let t17115 = 0.11038e0_f64 * t16868;
    let t17117 = 0.20128333333333333334e0_f64 * t16712;
    let t17131 = 0.22076e0_f64 * t16892;
    let t17140 = 0.13418888888888888889e0_f64 * t16708;
    let t17154 = t1749 * t3495;
    (t17050, t17052, t17066, t17075, t17089, t17092, t17097, t17115, t17117, t17131, t17140, t17154)
}
