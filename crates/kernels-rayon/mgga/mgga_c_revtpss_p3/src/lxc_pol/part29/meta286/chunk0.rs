//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1172/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1172(t1333: f64, t3860: f64, t4144: f64, t4147: f64, t30: f64, t513: f64, t33: f64, t516: f64, t2435: f64, t3900: f64, t212: f64, t4066: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9597 = t3860 * t1333;
    let t9599 = t4144 * t4147;
    let t9603 = t30 * t30;
    let t9605 = 1.0_f64 / t513 / t9603;
    let t9615 = t33 * t33;
    let t9617 = 1.0_f64 / t516 / t9615;
    let t9632 = t2435 * t3900;
    let t9634 = t212 * t4066;
    (t9597, t9599, t9605, t9617, t9632, t9634)
}
