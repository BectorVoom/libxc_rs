//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 404/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk404<F: Float>(t1399: F, t472: F, t92: F, t93: F, t427: F, t460: F, t40: F, t414: F, t428: F, t461: F, t409: F, t1: F, t467: F, t408: F, t413: F, t88: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1400 = t1399 * t472;
    let t1402 = 1.0 / t92;
    let t1412 = 1.0 / t93;
    let t1425 = t427 * t460;
    let t1426 = t40 * t1425;
    let t1428 = t414 * t428;
    let t1430 = t414 * t461;
    let t1431 = 8.0 * t1430;
    let t1432 = t409 * t428;
    let t1434 = t427 * t1;
    let t1435 = t1434 * t467;
    let t1438 = t408 * t413;
    let t1439 = t1438 * t88;
    (t1400, t1402, t1412, t1425, t1426, t1428, t1431, t1432, t1434, t1435, t1438, t1439)
}
