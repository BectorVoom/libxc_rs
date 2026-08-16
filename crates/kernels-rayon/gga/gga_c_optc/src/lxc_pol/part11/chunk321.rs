//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 321/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk321(t1331: f64, t1345: f64, t1347: f64, t1355: f64, t1360: f64, t1367: f64, t241: f64, t252: f64, t810: f64, t829: f64, t1366: f64, t828: f64, t837: f64) -> (f64, f64, f64) {
    let t1371 = t241 * (-0.3109e-1_f64 * t1347 * t252 + 1.0_f64 * t810 * t1355 + t1331 - t1345 - 0.19751789702565206229e-1_f64 * t1360 + 0.58482233974552040708e0_f64 * t829 * t1367);
    let t1373 = 0.19751789702565206229e-1_f64 * t241 * t1360;
    let t1375 = t828 * t1366 * t837;
    (t1371, t1373, t1375)
}
