//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1086/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1086(t355: f64, t7329: f64, t988: f64, t1015: f64, t115: f64, t18485: f64, t2326: f64, t2328: f64, t2337: f64, t23490: f64, t23495: f64, t23503: f64, t23510: f64, t23513: f64, t23519: f64, t23520: f64, t23523: f64, t23531: f64, t23537: f64, t2433: f64, t2554: f64, t279: f64, t363: f64, t5: f64, t7313: f64, t8289: f64, rho0: f64, sigma0: f64) -> f64 {
    let t23539 = t355 * t988 * t7329;
    let t23541 = -1520000.0_f64 / 243.0_f64 * t23490 + 136400.0_f64 / 243.0_f64 * t2554 * t7313 * t1015 - 400.0_f64 / 81.0_f64 * t23495 + 51260000.0_f64 / 729.0_f64 * t2326 * t2328 / t18485 * t2337 + 10472.0_f64 / 81.0_f64 * t355 * t23503 * t115 * t5 * t363 - 2464.0_f64 / 81.0_f64 * t23510 + 200.0_f64 / 81.0_f64 * t2433 * t23513 + 400000000.0_f64 / 6561.0_f64 * t23519 * t23520 * t8289 * sigma0 / t279 / t23523 / rho0 * t1015 - 16.0_f64 / 3.0_f64 * t23531 - t23537 - 160.0_f64 / 81.0_f64 * t23539;
    t23541
}
