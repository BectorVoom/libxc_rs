//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1200/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1200<F: Float>(t10140: F, t8360: F, t23382: F, t23389: F, t28324: F, t28333: F, t28335: F, t28345: F, t28353: F, t28364: F, t28374: F, t28376: F, t28380: F, t28384: F, t31693: F, t31736: F, t31765: F, t31787: F, t31822: F, t31868: F, t31892: F, t31940: F, t32016: F, t32057: F, t32097: F, t32137: F, t32168: F, t32183: F, t32208: F) -> (F,) {
    let t32215 = t8360 * t10140;
    let t32221 = -0.17149607247227894789e-2 * t28324 - t23382 - 0.48272968547752592737e-2 * t28333 - 0.28963781128651555642e-1 * t28335 - 0.27439371595564631662e-1 * t28345 + 0.51448821741683684368e-2 * t28353 - 0.10289764348336736874e-1 * t28364 + 0.22866142996303859718e-2 * t32215 - 11.0 / 108.0 * t28374 - t28376 / 27.0 - t28380 / 96.0 + t28384 / 18.0 - t23389;
    let t32225 = t31693 + t31736 + t31765 + t31787 + t31822 + t31868 + t31892 + t31940 + t32016 + t32057 + t32097 + t32137 + t32168 + t32183 + t32208 + t32221;
    (t32225,)
}
