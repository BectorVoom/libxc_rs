//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 349/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk349(t1119: f64, t1124: f64, t422: f64, t418: f64) -> (f64, f64, f64, f64) {
    let t1126 = -t1119 + 0.17808333333333333333e-1_f64 * t1124;
    let t1128 = 0.621814e-1_f64 * t1126 * t422;
    let t1129 = t418 * t418;
    let t1130 = 1.0_f64 / t1129;
    (t1126, t1128, t1129, t1130)
}
