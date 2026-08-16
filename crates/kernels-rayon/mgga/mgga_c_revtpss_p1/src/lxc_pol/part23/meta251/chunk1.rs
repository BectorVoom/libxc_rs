//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1437/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1437(t1337: f64, t9586: f64, t4146: f64, t565: f64, t1333: f64, t3860: f64, t30: f64, t513: f64, t33: f64, t516: f64, t2435: f64, t3900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9588 = 0.56968947174242584612e-3_f64 * t1337 * t9586;
    let t9593 = 1.0_f64 / t4146 / t565;
    let t9597 = t3860 * t1333;
    let t9598 = 36.0_f64 * t9597;
    let t9603 = t30 * t30;
    let t9605 = 1.0_f64 / t513 / t9603;
    let t9615 = t33 * t33;
    let t9617 = 1.0_f64 / t516 / t9615;
    let t9632 = t2435 * t3900;
    (t9588, t9593, t9597, t9598, t9603, t9605, t9615, t9617, t9632)
}
