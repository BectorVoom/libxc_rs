//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1034/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1034(t760: f64, t9318: f64, t2609: f64, t717: f64, t162: f64, t9544: f64, t158: f64, t755: f64, t9586: f64, t2619: f64, t2622: f64, t2629: f64, t9863: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10554 = 0.35089341735807877242e1_f64 * t760 * t9318;
    let t10563 = t717 * t2609;
    let t10565 = t162 * t9544;
    let t10566 = t158 * t10565;
    let t10568 = 0.56968947174242584612e-3_f64 * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10577 = 0.16265371950452609763e-1_f64 * t2629 * t9863;
    (t10554, t10563, t10566, t10568, t10569, t10577)
}
