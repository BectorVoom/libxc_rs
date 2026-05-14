//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 652/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk652<F: Float>(t8130: F, t935: F, t1725: F, t3085: F, t626: F, t934: F, t419: F, t3096: F, t3095: F, t8715: F, t11059: F, t3088: F, t11050: F, t1527: F, t11287: F, t8099: F, t8110: F, t8113: F, t8116: F, t8133: F) -> (F, F, F, F, F, F, F, F) {
    let t11294 = t8130 * t935;
    let t11296 = t1725 * t3085;
    let t11297 = 0.1134997482304526749e-1 * t11296;
    let t11298 = t626 * t934;
    let t11299 = t419 * t11298;
    let t11301 = t1725 * t3096;
    let t11303 = t8715 * t3095;
    let t11304 = t419 * t11303;
    let t11306 = t3088 * t11059;
    let t11307 = t419 * t11306;
    let t11309 = t1527 * t11050;
    let t11310 = t419 * t11309;
    let t11312 = -0.45399899292181069959e-1 * t11287 - 0.42562405586419753086e-2 * t8099 - 0.28374937057613168724e-2 * t8110 + 0.21281202793209876543e-2 * t8113 + 0.28374937057613168724e-2 * t8116 - 0.1134997482304526749e-1 * t8133 + 0.62424861526748971195e-1 * t11294 - t11297 - 0.14187468528806584362e-2 * t11299 - 0.68099848938271604939e-1 * t11301 - 0.2979368391049382716e-1 * t11304 - 0.51074886703703703704e-1 * t11307 + 0.38306165027777777778e-1 * t11310;
    (t11294, t11296, t11299, t11301, t11304, t11307, t11310, t11312)
}
