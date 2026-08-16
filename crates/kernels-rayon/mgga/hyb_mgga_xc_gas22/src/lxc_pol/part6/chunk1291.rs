//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1291/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1291(t1289: f64, t3138: f64, t3142: f64, t8497: f64, t8512: f64, t8518: f64, t10200: f64, t8498: f64, t10195: f64, t10199: f64, t10205: f64, t10212: f64, t1240: f64, t1318: f64, t2002: f64, t20560: f64, t20563: f64, t20574: f64, t20578: f64, t20579: f64, t23909: f64, t27852: f64, t27871: f64, t3: f64, t3139: f64, t3140: f64, t3271: f64, t675: f64, t8441: f64, t8502: f64, t8506: f64, t8511: f64, t8513: f64, t8514: f64, t8519: f64, t8521: f64, t8526: f64) -> f64 {
    let t27996 = t3138 * t8497 * t1289 * t3142;
    let t28005 = t8512 * t1289;
    let t28009 = t8518 * t1289;
    let t28014 = t3138 * t8498 * t10200;
    let t28043 = t3138 * t23909 * t10212 / 6.0_f64 - t27996 / 36.0_f64 - t3138 * t3139 * t3271 * t3142 / 12.0_f64 - t3138 * t10195 * t8506 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t8511 * t28005 * t8514 + t3138 * t28009 * t8521 / 6.0_f64 - t28014 / 36.0_f64 + t8526 * t3140 * t27871 / 16.0_f64 - t3138 * t8502 * t10200 / 12.0_f64 - t3138 * t3140 * t10199 * t2002 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t8511 * t8513 * t27852 + t3138 * t8519 * t1318 * t3 * t675 / 6.0_f64 + t8526 * t8502 * t10205 / 8.0_f64 - 3.0_f64 / 32.0_f64 * t1240 * t8441 + t20560 / 48.0_f64 + t20563 / 96.0_f64 - 5.0_f64 / 144.0_f64 * t20574 + t20578 + t20579 / 96.0_f64;
    t28043
}
