//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1404/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1404(t26257: f64, t6427: f64, t20433: f64, t6952: f64, t12289: f64, t20490: f64, t6936: f64, t20495: f64, t3788: f64, t1339: f64, t20568: f64, t20501: f64, t6916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t107107 = t26257 * t6427;
    let t107109 = t6952 * t20433;
    let t107112 = t6936 * t12289 * t20490;
    let t107115 = t6936 * t3788 * t20495;
    let t107118 = t6936 * t1339 * t20568;
    let t107120 = t6916 * t20501;
    (t107107, t107109, t107112, t107115, t107118, t107120)
}
