//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1166/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1166(t1160: f64, t5117: f64, t1737: f64, t3476: f64, t16868: f64, t16712: f64, t16892: f64, t16708: f64, t1179: f64, t5155: f64, t1719: f64, t3383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17026 = t5117 * t1160;
    let t17032 = t1737 * t3476;
    let t17050 = 0.13892666666666666667e0_f64 * t16868;
    let t17052 = 0.34431666666666666666e0_f64 * t16712;
    let t17066 = 0.27785333333333333334e0_f64 * t16892;
    let t17075 = 0.22954444444444444444e0_f64 * t16708;
    let t17089 = t5155 * t1179;
    let t17092 = t1719 * t3383;
    (t17026, t17032, t17050, t17052, t17066, t17075, t17089, t17092)
}
