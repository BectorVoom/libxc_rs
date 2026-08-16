//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1340/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1340(t1749: f64, t3520: f64, t16868: f64, t16712: f64, t16892: f64, t16708: f64, t3495: f64, t1770: f64, t3781: f64, t1284: f64, t1811: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17097 = t1749 * t3520;
    let t17115 = 0.11038e0_f64 * t16868;
    let t17117 = 0.20128333333333333334e0_f64 * t16712;
    let t17131 = 0.22076e0_f64 * t16892;
    let t17140 = 0.13418888888888888889e0_f64 * t16708;
    let t17154 = t1749 * t3495;
    let t17183 = t1770 * t3781;
    let t17191 = t1284 * t1811;
    let t17192 = t1209 * t17191;
    (t17097, t17115, t17117, t17131, t17140, t17154, t17183, t17192)
}
