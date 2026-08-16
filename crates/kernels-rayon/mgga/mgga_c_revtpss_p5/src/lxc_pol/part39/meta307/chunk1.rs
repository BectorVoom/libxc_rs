//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1076/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1076(t1033: f64, t11267: f64, t3169: f64, t3173: f64, t2866: f64, t914: f64, t2923: f64, t910: f64, t287: f64, t2922: f64, t275: f64, t11132: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11268 = t1033 * t11267;
    let t11271 = t3169 * t3173;
    let t11289 = t2866 * t914;
    let t11294 = t910 * t2923;
    let t11298 = 1.0_f64 / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11304 = 28.0_f64 / 27.0_f64 * t11132;
    (t11268, t11271, t11289, t11294, t11299, t11304)
}
