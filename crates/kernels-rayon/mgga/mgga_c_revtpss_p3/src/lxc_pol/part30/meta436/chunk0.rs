//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1667/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1667(t1139: f64, t16926: f64, t16710: f64, t5095: f64, t698: f64, t1132: f64, t16708: f64, t16717: f64, t16722: f64, t16735: f64, t16740: f64, t16744: f64, t16908: f64) -> (f64, f64, f64, f64) {
    let t16927 = t1139 * t16926;
    let t16929 = 0.39862222222222222222e0_f64 * t16710;
    let t16931 = t698 * t5095;
    let t16933 = t1132 * t16926;
    let t16940 = 0.36514074074074074075e-1_f64 * t16908 + 0.3071625e0_f64 * t16927 - t16929 + 0.13287407407407407408e0_f64 * t16708 + 0.36514074074074074074e-1_f64 * t16931 + 0.1898925e1_f64 * t16933 - 0.11958666666666666667e1_f64 * t16722 + 0.11958666666666666667e1_f64 * t16740 + 0.59793333333333333334e0_f64 * t16744 + 0.17938e1_f64 * t16735 + 0.33218518518518518518e0_f64 * t16717;
    (t16927, t16931, t16933, t16940)
}
