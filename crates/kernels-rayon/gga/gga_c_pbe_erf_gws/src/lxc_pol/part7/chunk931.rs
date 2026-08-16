//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 931/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk931(t5352: f64, t636: f64, t4923: f64, t5218: f64, t5220: f64, t5334: f64, t561: f64, t582: f64, t1403: f64, t1406: f64, t1663: f64, t1820: f64, t2559: f64) -> (f64, f64, f64, f64) {
    let t17390 = t5352 * t636;
    let t17391 = 32.0_f64 / 15.0_f64 * t17390;
    let t17394 = 64.0_f64 / 15.0_f64 * t5218 * t5220 * t4923;
    let t17396 = t561 * t582 * t5334;
    let t17397 = 32.0_f64 / 45.0_f64 * t17396;
    let t17402 = 16.0_f64 / 9.0_f64 * t1820 * t2559 * t1406 * t1663 * t1403;
    (t17391, t17394, t17397, t17402)
}
