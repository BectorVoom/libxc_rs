//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1330/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1330<F: Float>(t10140: F, t8360: F, t23382: F, t23389: F, t28324: F, t28333: F, t28335: F, t28345: F, t28353: F, t28364: F, t28374: F, t28376: F, t28380: F, t28384: F) -> F {
    let t32215 = t8360 * t10140;
    let t32221 = -F::cast_from(0.17149607247227894789e-2_f64) * t28324 - t23382 - F::cast_from(0.48272968547752592737e-2_f64) * t28333 - F::cast_from(0.28963781128651555642e-1_f64) * t28335 - F::cast_from(0.27439371595564631662e-1_f64) * t28345 + F::cast_from(0.51448821741683684368e-2_f64) * t28353 - F::cast_from(0.10289764348336736874e-1_f64) * t28364 + F::cast_from(0.22866142996303859718e-2_f64) * t32215 - F::new(11.0) / F::new(108.0) * t28374 - t28376 / F::new(27.0) - t28380 / F::new(96.0) + t28384 / F::new(18.0) - t23389;
    t32221
}
