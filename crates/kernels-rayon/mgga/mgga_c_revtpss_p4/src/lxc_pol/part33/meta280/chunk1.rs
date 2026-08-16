//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1251/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1251(t197: f64, t532: f64, t1450: f64, t2033: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64) -> (f64, f64, f64, f64) {
    let t8995 = t197 * t532;
    let t8996 = t2033 * t1450;
    let t9273 = 1.0_f64 / t2580 / t143;
    let t9274 = t130 * t9273;
    let t9275 = t2566 * t700;
    let t9276 = t9275 * t2584;
    let t9278 = 0.96491876992155210402e2_f64 * t9274 * t9276;
    (t8995, t8996, t9275, t9278)
}
