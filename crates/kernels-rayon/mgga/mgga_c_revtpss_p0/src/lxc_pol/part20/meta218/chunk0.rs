//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1004/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1004(t10563: f64, t162: f64, t9544: f64, t158: f64, t755: f64, t9586: f64, t2619: f64, t2622: f64, t10552: f64, t10554: f64, t10557: f64, t10560: f64, t10562: f64, t9333: f64, t9394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10564 = 3.0_f64 * t10563;
    let t10565 = t162 * t9544;
    let t10566 = t158 * t10565;
    let t10568 = 0.56968947174242584612e-3_f64 * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10570 = 0.73245789224026180216e-3_f64 * t10569;
    let t10571 = t9333 - t10552 + t10554 + t10557 + t9394 + t10560 + t10562 + t10564 + t10566 - t10568 + t10570;
    (t10564, t10565, t10566, t10568, t10570, t10571)
}
