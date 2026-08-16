//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1266/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1266(t13800: f64, t9736: f64, t241: f64, t820: f64, t9991: f64, t5697: f64, t9962: f64, t5701: f64, t5608: f64, t5675: f64, t9934: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13801 = t9736 * t13800;
    let t13804 = t820 * t9991 * t241;
    let t13810 = t9962 * t5697;
    let t13813 = 0.20007875121765877254e-2_f64 * t9962 * t5701;
    let t13829 = t5608 * t5675;
    let t13830 = t9934 * t13829;
    let t13832 = 0.28582678745379824648e-4_f64 * t2661 * t13830;
    (t13801, t13804, t13810, t13813, t13829, t13832)
}
