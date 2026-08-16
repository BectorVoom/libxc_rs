//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1039/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1039(t2710: f64, t2793: f64, t9285: f64, t2470: f64, t2804: f64, t874: f64, t875: f64, t9288: f64, t2718: f64, t860: f64, t243: f64, t816: f64, t9707: f64) -> (f64, f64, f64, f64, f64) {
    let t10645 = 0.46263278077393568556e-2_f64 * t2710 * t2793 * t9285;
    let t10647 = t874 * t2804 * t2470;
    let t10651 = 0.30356481678079769392e-1_f64 * t874 * t875 * t9288;
    let t10661 = t2718 * t860;
    let t10671 = t9707 * t243 * t816;
    (t10645, t10647, t10651, t10661, t10671)
}
