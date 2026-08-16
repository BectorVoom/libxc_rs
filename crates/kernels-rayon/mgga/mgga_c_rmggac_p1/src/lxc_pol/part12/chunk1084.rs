//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1084/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1084(t9088: f64, t9093: f64, t9102: f64, t9112: f64, t9114: f64, t9119: f64, t37039: f64, t7913: f64, t7916: f64, t7918: f64, t8222: f64, t9107: f64, t9636: f64) -> (f64, f64, f64) {
    let t42316 = 0.85129199786595678796e-5_f64 * t9088;
    let t42317 = 0.39914139006212695214e-1_f64 * t9093;
    let t42320 = 0.11974241701863808564e0_f64 * t9102;
    let t42322 = 0.85129199786595678796e-5_f64 * t9112;
    let t42323 = 0.85129199786595678796e-5_f64 * t9114;
    let t42324 = 0.31923449919973379548e-4_f64 * t9119;
    let t42325 = -t8222 - t7913 - t42320 + t7916 + t7918 + 0.25538759935978703638e-4_f64 * t9107 + t42322 - t42323 - t42324 + t9636 + t37039;
    (t42316, t42317, t42325)
}
