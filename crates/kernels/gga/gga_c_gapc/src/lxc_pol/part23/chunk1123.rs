//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1123/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1123<F: Float>(t34295: F, t34303: F, t34308: F, t34313: F, t35369: F, t35375: F, t35378: F, t35714: F, t36055: F, t36058: F, t36067: F, t36072: F, t36074: F, t36078: F, t36080: F, t11283: F) -> (F, F) {
    let t36086 = t34295 - t34303 + t34308 + t34313 - t35369 - t35375 + t35378 + t35714 - t36055 + t36058 - t36067 + t36072 + t36074 + t36078 - t36080;
    let t36089 = 4.0 * t11283;
    (t36086, t36089)
}
