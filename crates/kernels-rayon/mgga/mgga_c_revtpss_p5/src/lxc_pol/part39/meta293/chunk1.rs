//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1045/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1045(t2710: f64, t2793: f64, t9285: f64, t2470: f64, t2804: f64, t874: f64, t875: f64, t9288: f64, t251: f64, t2722: f64, t2723: f64, t4503: f64) -> (f64, f64, f64, f64, f64) {
    let t10645 = 0.46263278077393568556e-2_f64 * t2710 * t2793 * t9285;
    let t10647 = t874 * t2804 * t2470;
    let t10651 = 0.30356481678079769392e-1_f64 * t874 * t875 * t9288;
    let t10652 = t251 * t2722;
    let t10654 = t4503 * t10652 * t2723;
    (t10645, t10647, t10651, t10652, t10654)
}
