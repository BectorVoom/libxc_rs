//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1571/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1571(t1737: f64, t3476: f64, t16868: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16727: f64, t16748: f64, t16871: f64, t16876: f64) -> (f64, f64) {
    let t17032 = t1737 * t3476;
    let t17050 = 0.13892666666666666667e0_f64 * t16868;
    let t17052 = 0.34431666666666666666e0_f64 * t16712;
    let t17061 = -t17050 + 0.104195e0_f64 * t16871 - t17052 + 0.516475e0_f64 * t16748 + 0.22954444444444444444e0_f64 * t16706 + 0.11577222222222222222e0_f64 * t16876 + 0.11477222222222222222e0_f64 * t12299 + 0.45908888888888888888e0_f64 * t12297 - 0.34431666666666666666e0_f64 * t12301 - 0.17215833333333333333e0_f64 * t12303 - 0.68863333333333333334e0_f64 * t16727;
    (t17032, t17061)
}
