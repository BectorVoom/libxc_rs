//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1728/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1728(t13789: f64, t22298: f64, t14038: f64, t14040: f64, t14042: f64, t14043: f64, t14049: f64, t14053: f64, t14057: f64, t1410: f64, t22285: f64, t22289: f64, t22292: f64, t22295: f64, t3934: f64, t9977: f64) -> (f64, f64) {
    let t22299 = t13789 * t22298;
    let t22304 = -0.20007875121765877254e-2_f64 * t22285 + 0.42874018118069736972e-2_f64 * t1410 * t22289 + 0.10003937560882938627e-2_f64 * t22292 - 0.85748036236139473945e-2_f64 * t3934 * t22295 + 0.17149607247227894789e-2_f64 * t3934 * t22299 - t14038 - t14040 + t14042 + 0.27104001498285508386e-3_f64 * t14043 - t14049 + t14053 - t14057 + 0.13552000749142754193e-3_f64 * t9977;
    (t22299, t22304)
}
