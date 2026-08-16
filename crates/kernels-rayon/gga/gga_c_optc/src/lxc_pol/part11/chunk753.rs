//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 753/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk753(t1359: f64, t2472: f64, t1329: f64, t2372: f64, t1347: f64, t2492: f64, t2517: f64, t2529: f64, t2415: f64, t1434: f64, t7274: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10409 = t1359 * t2472;
    let t10416 = t1329 * t2372;
    let t10419 = t1347 * t2492;
    let t10478 = t1347 * t2517;
    let t10485 = t1359 * t2529;
    let t10493 = t1329 * t2415;
    let t10594 = t7274 * t1434;
    let t10595 = t999 * t10594;
    (t10409, t10416, t10419, t10478, t10485, t10493, t10594, t10595)
}
