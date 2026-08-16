//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 916/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk916(t4810: f64, t8012: f64, t8014: f64, t3361: f64, t409: f64, t414: f64, t8016: f64, t8018: f64, t8023: f64, t4819: f64, t4688: f64, t4711: f64, t4714: f64, t4718: f64, t4815: f64, t8011: f64, t8022: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10254 = 0.24415406715670879921e-3_f64 * t4810;
    let t10255 = 16.0_f64 * t8012;
    let t10256 = 24.0_f64 * t8014;
    let t10257 = t409 * t3361;
    let t10258 = 4.0_f64 * t10257;
    let t10259 = t414 * t3361;
    let t10260 = 4.0_f64 * t10259;
    let t10261 = 0.11696446794910408142e1_f64 * t8016;
    let t10262 = 0.346315117987517266e2_f64 * t8018;
    let t10263 = 0.23392893589820816284e1_f64 * t8023;
    let t10264 = 8.0_f64 * t4819;
    let t10265 = t10254 - t4815 + t4688 + t4711 - t4714 - t4718 - t8011 - t10255 - t10256 + t10258 - t10260 - t10261 - t10262 - t8022 + t10263 - t10264;
    (t10254, t10255, t10256, t10258, t10260, t10261, t10262, t10263, t10264, t10265)
}
