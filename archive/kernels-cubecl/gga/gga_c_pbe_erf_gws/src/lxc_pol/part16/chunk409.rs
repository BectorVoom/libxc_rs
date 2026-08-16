//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 409/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk409<F: Float>(t1422: F, t87: F, t40: F, t427: F, t460: F, t414: F, t428: F, t461: F, t409: F, t1: F, t467: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1423 = t1422 * t87;
    let t1424 = t40 * t1423;
    let t1425 = t427 * t460;
    let t1426 = t40 * t1425;
    let t1427 = F::cast_from(2.0_f64) * t1426;
    let t1428 = t414 * t428;
    let t1429 = F::cast_from(8.0_f64) * t1428;
    let t1430 = t414 * t461;
    let t1431 = F::cast_from(8.0_f64) * t1430;
    let t1432 = t409 * t428;
    let t1433 = F::cast_from(8.0_f64) * t1432;
    let t1434 = t427 * t1;
    let t1435 = t1434 * t467;
    (t1423, t1424, t1425, t1426, t1427, t1428, t1429, t1431, t1432, t1433, t1434, t1435)
}
