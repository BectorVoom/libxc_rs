//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 896/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk896<F: Float>(t10350: F, t10356: F, t10360: F, t10362: F, t10364: F, t10369: F, t10371: F, t10375: F, t10377: F, t10382: F, t10387: F, t10391: F, t10396: F, t10399: F, t5906: F, t5912: F) -> (F,) {
    let t11200 = t10350 + t10356 + t10360 + t10362 + t10364 - t10369 + t10371 + t10375 + t10377 + t10382 - t10387 + t10391 + t5906 + 4.0 / 3.0 * t5912 + t10396 - t10399;
    (t11200,)
}
