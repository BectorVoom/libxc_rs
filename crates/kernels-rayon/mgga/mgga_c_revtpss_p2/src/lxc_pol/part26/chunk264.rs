//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 264/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk264(t903: f64, t908: f64, t291: f64, t287: f64, t275: f64, t276: f64, t902: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t910 = -t903 - 0.17808333333333333333e-1_f64 * t908;
    let t912 = 0.621814e-1_f64 * t910 * t291;
    let t913 = t287 * t287;
    let t914 = 1.0_f64 / t913;
    let t915 = t275 * t914;
    let t916 = 1.0_f64 / t276;
    let t918 = -t902 / 3.0_f64 - t908 / 3.0_f64;
    (t910, t912, t913, t914, t915, t916, t918)
}
