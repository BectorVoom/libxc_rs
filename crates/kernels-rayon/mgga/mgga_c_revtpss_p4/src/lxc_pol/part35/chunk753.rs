//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 753/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk753(t760: f64, t9419: f64, t9387: f64, t9372: f64, t9425: f64, t2475: f64, t73: f64, t2710: f64, t2793: f64, t9285: f64, t874: f64, t875: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10592 = 0.10389515463408878255e3_f64 * t760 * t9419;
    let t10596 = 0.5848223622634646207e0_f64 * t760 * t9387;
    let t10604 = 0.10254018858216406658e4_f64 * t760 * t9372;
    let t10611 = 0.35089341735807877242e1_f64 * t760 * t9425;
    let t10626 = t73 * t2475;
    let t10645 = 0.46263278077393568556e-2_f64 * t2710 * t2793 * t9285;
    let t10651 = 0.30356481678079769392e-1_f64 * t874 * t875 * t9288;
    (t10592, t10596, t10604, t10611, t10626, t10645, t10651)
}
