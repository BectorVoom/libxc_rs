//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 847/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk847<F: Float>(t10411: F, t5218: F, t10356: F, t10360: F, t10362: F, t10364: F, t10369: F, t10371: F, t10375: F, t10377: F, t10382: F, t10387: F, t10391: F, t10396: F, t10399: F, t10400: F, t10405: F, t10410: F) -> (F, F) {
    let t10413 = 16.0 / 45.0 * t5218 * t10411;
    let t10414 = t10356 + t10360 + t10362 + t10364 - t10369 + t10371 + t10375 + t10377 + t10382 - t10387 + t10391 + t10396 - t10399 - t10400 + t10405 + t10410 - t10413;
    (t10413, t10414)
}
