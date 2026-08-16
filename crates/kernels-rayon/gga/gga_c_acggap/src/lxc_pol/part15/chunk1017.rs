//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1017/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1017(t2327: f64, t7630: f64, t13287: f64, t31057: f64, t35700: f64, t1429: f64, t7605: f64, t31593: f64, t30219: f64, t8469: f64, t1562: f64, t31824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35744 = t7630 * t2327;
    let t35747 = t31057 * t13287 * t35700;
    let t35755 = t7605 * t1429;
    let t35764 = 0.42874018118069736972e-3_f64 * t31593;
    let t35774 = t30219 * t8469;
    let t35784 = t31824 * t1562;
    (t35744, t35747, t35755, t35764, t35774, t35784)
}
