//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1093/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1093(t5697: f64, t9962: f64, t5701: f64, t5608: f64, t5675: f64, t9934: f64, t2661: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t13810 = t9962 * t5697;
    let t13813 = 0.20007875121765877254e-2_f64 * t9962 * t5701;
    let t13829 = t5608 * t5675;
    let t13830 = t9934 * t13829;
    let t13832 = 0.28582678745379824648e-4_f64 * t2661 * t13830;
    let t13845 = t2482 * t4000 * t814;
    let t13846 = t550 * t136;
    (t13810, t13813, t13832, t13845, t13846)
}
