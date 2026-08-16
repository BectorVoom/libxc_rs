//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1204/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1204(t2251: f64, t2276: f64, t6383: f64, t6: f64, t6385: f64, t2306: f64, t6277: f64, t20504: f64, t3065: f64, t858: f64, t8988: f64, t6217: f64, t6411: f64) -> (f64, f64, f64, f64, f64) {
    let t21399 = t2276 * t2251 * t6383;
    let t21400 = t6 * t6385;
    let t21405 = t2306 * t6277;
    let t21410 = t3065 * t858 * t20504;
    let t21412 = t8988 * t21410 / 4.0_f64;
    let t21414 = t6217 * t6411 / 16.0_f64;
    (t21399, t21400, t21405, t21412, t21414)
}
