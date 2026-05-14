//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 839/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk839<F: Float>(t10290: F, t10291: F, t10294: F, t10299: F, t10302: F, t10303: F, t10305: F, t4872: F, t4873: F, t4876: F, t4910: F, t6971: F, t6998: F, t7045: F, t7047: F, t7074: F, t7075: F) -> (F,) {
    let t10306 = -t4872 - t6971 + 4.0 / 135.0 * t6998 + 0.33245444444444444444e-1 * t4873 + t4876 + t10290 + t4910 - t10291 - t7045 + t7047 + t7074 + 8.0 / 9.0 * t7075 + t10294 + t10299 - t10302 - t10303 - t10305;
    (t10306,)
}
