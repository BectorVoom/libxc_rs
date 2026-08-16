//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 753/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk753<F: Float>(t1359: F, t2472: F, t1329: F, t2372: F, t1347: F, t2492: F, t2517: F, t2529: F, t2415: F, t1434: F, t7274: F, t999: F) -> (F, F, F, F, F, F, F, F) {
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
