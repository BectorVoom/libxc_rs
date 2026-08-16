//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1394/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1394(t14563: f64, t2798: f64, t1568: f64, t2783: f64, t786: f64, t2801: f64, t233: f64, t4469: f64, t869: f64, t689: f64, t2435: f64, t4519: f64) -> (f64, f64, f64, f64, f64) {
    let t14564 = t2798 * t14563;
    let t14567 = t2783 * t1568;
    let t14568 = t786 * t14567;
    let t14570 = 0.19514881078765566038e-1_f64 * t14568 * t2801;
    let t14574 = t233 * t4469;
    let t14575 = t869 * t14574;
    let t14577 = 0.10975748638225852664e-1_f64 * t689 * t14575;
    let t14581 = t2435 * t4519;
    (t14564, t14568, t14570, t14577, t14581)
}
