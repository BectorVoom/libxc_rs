//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 353/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk353<F: Float>(t1396: F, t470: F, t92: F, t93: F, t414: F, t461: F, t408: F, t413: F) -> (F, F, F, F, F) {
    let t1397 = t470 * t1396;
    let t1398 = 0.58482233974552040708e0 * t1397;
    let t1402 = 1.0 / t92;
    let t1412 = 1.0 / t93;
    let t1430 = t414 * t461;
    let t1431 = 8.0 * t1430;
    let t1438 = t408 * t413;
    (t1398, t1402, t1412, t1431, t1438)
}
