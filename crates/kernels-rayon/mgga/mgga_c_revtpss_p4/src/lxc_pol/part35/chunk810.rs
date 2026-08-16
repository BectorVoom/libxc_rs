//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 810/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk810(t1719: f64, t3432: f64, t1729: f64, t2439: f64, t1737: f64, t3451: f64, t3476: f64, t3383: f64, t1749: f64, t3520: f64, t3495: f64, t1770: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16840 = t1719 * t3432;
    let t16876 = t2439 * t1729;
    let t17023 = t1737 * t3451;
    let t17032 = t1737 * t3476;
    let t17092 = t1719 * t3383;
    let t17097 = t1749 * t3520;
    let t17154 = t1749 * t3495;
    let t17183 = t1770 * t3781;
    (t16840, t16876, t17023, t17032, t17092, t17097, t17154, t17183)
}
