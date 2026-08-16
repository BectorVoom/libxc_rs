//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1241/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1241(t2453: f64, t3908: f64, t7911: f64, t136: f64, t2457: f64, t7920: f64, t94589: f64, t2435: f64, t27965: f64, t14090: f64, t26054: f64, t10073: f64, t1903: f64, t2029: f64, t25929: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97810 = t2453 * t7911 * t3908;
    let t97814 = t7920 * t136 * t2457;
    let t97815 = t94589 * t97814;
    let t97823 = t2435 * t27965;
    let t97825 = t26054 * t14090;
    let t97847 = t10073 * t25929 * t2029 * t1903;
    (t97810, t97814, t97815, t97823, t97825, t97847)
}
