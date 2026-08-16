//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1566/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1566(t19380: f64, t373: f64, t371: f64, t372: f64, t19463: f64, t366: f64, t3094: f64, t4186: f64, t4781: f64, t3092: f64, t4786: f64, t6092: f64) -> (f64, f64, f64, f64) {
    let t19768 = t373 * t19380;
    let t19770 = t371 * t372 * t19768;
    let t19773 = t19463 * t366;
    let t19776 = t3094 * t4186;
    let t19777 = t4781 * t19776;
    let t19778 = t3092 * t19777;
    let t19781 = t6092 * t4786;
    (t19770, t19773, t19778, t19781)
}
