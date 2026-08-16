//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 614/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk614(t3204: f64, t366: f64, t1024: f64, t1053: f64, t1026: f64, t127: f64, t371: f64, t1025: f64, t225: f64, t3046: f64) -> (f64, f64, f64, f64, f64) {
    let t3205 = t3204 * t366;
    let t3211 = t1024 * t1053;
    let t3215 = t371 * t127 * t1026;
    let t3216 = t1025 * t3215;
    let t3223 = t3046 * t225;
    (t3205, t3211, t3215, t3216, t3223)
}
