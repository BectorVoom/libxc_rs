//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1022/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1022(t4358: f64, t461: f64, t409: f64, t4832: f64, t18467: f64, t18471: f64, t18474: f64, t18477: f64, t18479: f64, t18512: f64, t18514: f64, t18518: f64, t18521: f64, t18523: f64, t18527: f64) -> (f64, f64, f64) {
    let t18528 = t4358 * t461;
    let t18529 = 96.0_f64 * t18528;
    let t18530 = t409 * t4832;
    let t18531 = 16.0_f64 * t18530;
    let t18532 = t18467 - t18471 - t18474 + t18477 + t18479 + t18512 - t18514 + t18518 + t18521 - t18523 + t18527 + t18529 + t18531;
    (t18529, t18531, t18532)
}
