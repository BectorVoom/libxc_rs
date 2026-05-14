//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 990/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk990<F: Float>(t19898: F, t2382: F, t19875: F, t19878: F, t19880: F, t19888: F, t19890: F, t19892: F, t19895: F, t2373: F, t2408: F, t2409: F, t4390: F, t4397: F, t4459: F, t4464: F, t4484: F, t6112: F, t6138: F, t6797: F, t8734: F) -> (F,) {
    let t19899 = t2382 * t19898;
    let t19904 = -t2408 * t2409 * t8734 * t6138 / 2.0 + 35.0 / 36.0 * t19875 + 35.0 / 72.0 * t19878 - 35.0 / 36.0 * t19880 - t6112 * t2373 / 12.0 - t4397 * t4459 / 8.0 - t4397 * t4464 / 24.0 - 7.0 / 12.0 * t19888 + 7.0 / 12.0 * t19890 - 7.0 / 12.0 * t19892 + t19895 * t6797 / 4.0 + t19899 * t4390 / 4.0 + t19899 * t4484 / 8.0;
    (t19904,)
}
