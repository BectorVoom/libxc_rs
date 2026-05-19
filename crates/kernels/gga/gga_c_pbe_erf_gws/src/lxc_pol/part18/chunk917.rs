//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 917/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk917<F: Float>(t4810: F, t8012: F, t8014: F, t3361: F, t409: F, t414: F, t8016: F, t8018: F, t8023: F, t4819: F, t4688: F, t4711: F, t4714: F, t4718: F, t4815: F, t8011: F, t8022: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10254 = F::cast_from(0.24415406715670879921e-3_f64) * t4810;
    let t10255 = F::new(16.0) * t8012;
    let t10256 = F::new(24.0) * t8014;
    let t10257 = t409 * t3361;
    let t10258 = F::new(4.0) * t10257;
    let t10259 = t414 * t3361;
    let t10260 = F::new(4.0) * t10259;
    let t10261 = F::cast_from(0.11696446794910408142e1_f64) * t8016;
    let t10262 = F::cast_from(0.346315117987517266e2_f64) * t8018;
    let t10263 = F::cast_from(0.23392893589820816284e1_f64) * t8023;
    let t10264 = F::new(8.0) * t4819;
    let t10265 = t10254 - t4815 + t4688 + t4711 - t4714 - t4718 - t8011 - t10255 - t10256 + t10258 - t10260 - t10261 - t10262 - t8022 + t10263 - t10264;
    (t10254, t10255, t10256, t10258, t10260, t10261, t10262, t10263, t10264, t10265)
}
