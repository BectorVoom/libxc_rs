//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 355/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk355(t1396: f64, t470: f64, t92: f64, t93: f64, t414: f64, t461: f64, t408: f64, t413: f64) -> (f64, f64, f64, f64, f64) {
    let t1397 = t470 * t1396;
    let t1398 = 0.58482233974552040708e0_f64 * t1397;
    let t1402 = 1.0_f64 / t92;
    let t1412 = 1.0_f64 / t93;
    let t1430 = t414 * t461;
    let t1431 = 8.0_f64 * t1430;
    let t1438 = t408 * t413;
    (t1398, t1402, t1412, t1431, t1438)
}
