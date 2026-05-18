//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1355/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1355<F: Float>(t15559: F, t4414: F, t11396: F, t12204: F, t14185: F, t2408: F, t3066: F, t54896: F, t54902: F, t54904: F, t55297: F, t56113: F, t56116: F, t56119: F, t56124: F, t56126: F, t56129: F, t56142: F, t56147: F, t56153: F, t9283: F) -> F {
    let t57984 = t4414 * t15559;
    let t57994 = -t56113 / F::new(24.0) + t56116 / F::new(24.0) + t56119 / F::new(8.0) + t56124 / F::new(48.0) - F::new(7.0) / F::new(576.0) * t56126 - t56129 / F::new(384.0) - F::new(7.0) / F::new(144.0) * t56142 - F::new(7.0) / F::new(144.0) * t56147 - t54896 + t56153 / F::new(24.0) + F::new(7.0) / F::new(36.0) * t57984 - t2408 * t9283 * t14185 * t11396 / F::new(24.0) + t3066 * t9283 * t55297 * t12204 / F::new(4.0) - t54902 + t54904;
    t57994
}
