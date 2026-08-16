//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1138/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1138(t6411: f64, t6538: f64, t6305: f64, t6402: f64, t6355: f64, t814: f64, t2387: f64, t6644: f64, t6648: f64, t2306: f64, t6643: f64, t2382: f64) -> (f64, f64, f64, f64, f64) {
    let t20400 = t6538 * t6411 / 16.0_f64;
    let t20401 = t6402 * t6305;
    let t20403 = t6355 * t814;
    let t20408 = t2387 * t6644;
    let t20410 = t20408 * t6648 / 8.0_f64;
    let t20411 = t2306 * t6643;
    let t20412 = t2382 * t20411;
    (t20400, t20401, t20403, t20410, t20412)
}
