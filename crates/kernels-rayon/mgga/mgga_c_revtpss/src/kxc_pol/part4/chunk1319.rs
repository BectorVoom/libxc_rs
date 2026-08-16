//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1319/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1319(t1149: f64, t5105: f64, t3384: f64, t1733: f64, t3427: f64, t3385: f64, t5108: f64, t12248: f64, t3435: f64, t5104: f64, t3433: f64, t12230: f64, t1732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16652 = t5105 * t1149;
    let t16654 = 4.0_f64 * t3384 * t16652;
    let t16655 = t1733 * t3427;
    let t16657 = 2.0_f64 * t3384 * t16655;
    let t16658 = t5108 * t3385;
    let t16660 = 0.96491876992155210402e2_f64 * t12248 * t16658;
    let t16661 = t5104 * t3435;
    let t16662 = t16661 * t1149;
    let t16664 = 0.32163958997385070134e2_f64 * t3433 * t16662;
    let t16665 = t5108 * t3427;
    let t16667 = 0.16081979498692535067e2_f64 * t3433 * t16665;
    let t16668 = t1732 * t12230;
    (t16654, t16657, t16660, t16664, t16667, t16668)
}
