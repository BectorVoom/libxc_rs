//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1330/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1330(t10140: f64, t8360: f64, t23382: f64, t23389: f64, t28324: f64, t28333: f64, t28335: f64, t28345: f64, t28353: f64, t28364: f64, t28374: f64, t28376: f64, t28380: f64, t28384: f64) -> f64 {
    let t32215 = t8360 * t10140;
    let t32221 = -0.17149607247227894789e-2_f64 * t28324 - t23382 - 0.48272968547752592737e-2_f64 * t28333 - 0.28963781128651555642e-1_f64 * t28335 - 0.27439371595564631662e-1_f64 * t28345 + 0.51448821741683684368e-2_f64 * t28353 - 0.10289764348336736874e-1_f64 * t28364 + 0.22866142996303859718e-2_f64 * t32215 - 11.0_f64 / 108.0_f64 * t28374 - t28376 / 27.0_f64 - t28380 / 96.0_f64 + t28384 / 18.0_f64 - t23389;
    t32221
}
