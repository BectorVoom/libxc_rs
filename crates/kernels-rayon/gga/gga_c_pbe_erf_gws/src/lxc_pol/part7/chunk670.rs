//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 670/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk670(t5346: f64, t561: f64, t1680: f64, t583: f64, t1781: f64, t631: f64, t184: f64, t221: f64, t1778: f64, t633: f64, t198: f64, t2735: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5347 = t561 * t5346;
    let t5348 = 8.0_f64 / 15.0_f64 * t5347;
    let t5349 = t1680 * t583;
    let t5350 = 8.0_f64 / 15.0_f64 * t5349;
    let t5351 = t1781 * t631;
    let t5352 = t5351 * t184;
    let t5354 = 4.0_f64 / 5.0_f64 * t5352 * t221;
    let t5355 = t633 * t1778;
    let t5356 = 4.0_f64 / 45.0_f64 * t5355;
    let t5357 = t2735 * t198;
    (t5348, t5350, t5351, t5352, t5354, t5356, t5357)
}
