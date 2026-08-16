//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 992/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk992(t10565: f64, t158: f64, t755: f64, t9586: f64, t2619: f64, t2622: f64, t2629: f64, t9863: f64, t123: f64, t752: f64, t2630: f64, t9866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10566 = t158 * t10565;
    let t10568 = 0.56968947174242584612e-3_f64 * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10577 = 0.16265371950452609763e-1_f64 * t2629 * t9863;
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    let t10582 = 0.48159733137676571078e0_f64 * t2629 * t9866;
    (t10566, t10568, t10569, t10577, t10579, t10582)
}
