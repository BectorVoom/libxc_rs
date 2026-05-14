//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 845/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk845<F: Float>(t10392: F, t418: F, t7063: F, t7062: F, t7069: F, t5117: F, t1044: F, t954: F, t422: F, t7505: F, t7115: F, t626: F, t7116: F, t661: F, t2566: F, t7495: F) -> (F, F, F, F, F, F, F, F) {
    let t10393 = t10392 * t418;
    let t10394 = t7063 * t10393;
    let t10396 = 16.0 / 45.0 * t7062 * t10394;
    let t10397 = t7069 * t10393;
    let t10399 = 8.0 / 27.0 * t7062 * t10397;
    let t10400 = 8.0 / 135.0 * t5117;
    let t10401 = t954 * t1044;
    let t10402 = t10401 * t422;
    let t10403 = t7505 * t10402;
    let t10405 = 16.0 / 45.0 * t7115 * t10403;
    let t10406 = t7116 * t626;
    let t10408 = t10406 * t10401 * t661;
    let t10410 = 16.0 / 45.0 * t7115 * t10408;
    let t10411 = t7495 * t2566;
    (t10396, t10399, t10400, t10401, t10402, t10405, t10410, t10411)
}
