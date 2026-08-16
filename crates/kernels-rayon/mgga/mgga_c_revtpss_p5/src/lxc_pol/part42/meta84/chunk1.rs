//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 495/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk495(t1196: f64, t1765: f64, t1201: f64, t1717: f64, t459: f64) -> (f64, f64, f64) {
    let t1767 = 0.5848223622634646207e0_f64 * t1196 * t1765;
    let t1769 = -t1201 + 0.83333333333333333333e-2_f64 * t1717;
    let t1770 = t1769 * t459;
    (t1767, t1769, t1770)
}
