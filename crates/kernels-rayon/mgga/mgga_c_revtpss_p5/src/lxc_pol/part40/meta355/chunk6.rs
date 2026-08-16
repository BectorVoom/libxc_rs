//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1223/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1223(t14605: f64, t2482: f64, t2801: f64, t10443: f64, t10552: f64, t10554: f64, t14312: f64, t14313: f64, t14315: f64, t14317: f64, t14324: f64, t14327: f64, t14329: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64) {
    let t14606 = t2482 * t14605;
    let t14608 = 0.19514881078765566038e-1_f64 * t14606 * t2801;
    let t14609 = t14312 + t14313 - t9278 + t9308 + t9316 + t10443 + t9329 + t9333 + t14315 + t14317 - t10552 + t10554 - t14324 + t14327 + t14329;
    (t14608, t14609)
}
