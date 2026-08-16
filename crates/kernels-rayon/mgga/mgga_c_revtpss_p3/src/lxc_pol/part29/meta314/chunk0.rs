//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1216/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1216(t755: f64, t9586: f64, t2619: f64, t2622: f64, t2390: f64, t72: f64, t757: f64, t2629: f64, t9863: f64, t123: f64, t752: f64, t2630: f64) -> (f64, f64, f64, f64, f64) {
    let t10568 = 0.56968947174242584612e-3_f64 * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10573 = t2390 * t72;
    let t10574 = t10573 * t757;
    let t10577 = 0.16265371950452609763e-1_f64 * t2629 * t9863;
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    (t10568, t10569, t10574, t10577, t10579)
}
