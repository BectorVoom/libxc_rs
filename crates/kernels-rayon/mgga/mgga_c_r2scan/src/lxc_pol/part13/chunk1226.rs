//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1226/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1226(t10986: f64, t40713: f64, t10635: f64, t40282: f64, t38323: f64, t38334: f64, t38337: f64, t38339: f64, t38342: f64, t38347: f64, t38350: f64, t38356: f64, t38359: f64, t38363: f64, t40699: f64, t40704: f64, t40708: f64, t40711: f64) -> (f64, f64, f64) {
    let t40715 = 5.0_f64 / 8.0_f64 * t40713 * t10986;
    let t40717 = 15.0_f64 / 8.0_f64 * t40282 * t10635;
    let t40718 = t38323 - 0.15243824895787514157e-3_f64 * t38334 + t38337 + 0.16260079888840015101e-2_f64 * t38339 - t38342 + t38347 - t38350 - t40699 - 0.38422568777328955684e-2_f64 * t38356 + 0.60975299583150056628e-3_f64 * t38359 + t38363 + t40704 + t40708 - t40711 - t40715 + t40717;
    (t40715, t40717, t40718)
}
