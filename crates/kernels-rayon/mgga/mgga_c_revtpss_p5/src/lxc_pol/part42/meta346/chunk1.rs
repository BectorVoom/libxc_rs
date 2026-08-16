//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1154/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1154(t1011: f64, t16060: f64, t3241: f64, t4924: f64, t12047: f64, t15905: f64, t12167: f64, t3057: f64, t380: f64, t3088: f64, t370: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16062 = t1011 * t16060 / 432.0_f64;
    let t16064 = t3241 * t4924 / 162.0_f64;
    let t16067 = t12047 * t15905;
    let t16081 = t12167 * t15905;
    let t16087 = t3057 * t380;
    let t16088 = t3088 * t370;
    let t16089 = t16087 * t16088;
    let t16094 = t994 * t380;
    (t16062, t16064, t16067, t16081, t16088, t16089, t16094)
}
