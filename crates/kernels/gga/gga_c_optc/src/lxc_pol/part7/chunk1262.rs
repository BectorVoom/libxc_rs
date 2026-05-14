//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1262/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1262<F: Float>(t26280: F, t26284: F, t26293: F, t26296: F, t26304: F, t26311: F, t26319: F, t26324: F, t26388: F, t26394: F, t26396: F, t26278: F, t26289: F, t26300: F, t26306: F, t26309: F, t26406: F, t26409: F, t26412: F, t26415: F, t26419: F, t26443: F, t26446: F) -> (F, F) {
    let t27901 = -0.1642e-2 * t26388 + 0.15510666666666666667e2 * t26319 - 0.5170222222222222222e1 * t26324 - 0.19704e-1 * t26394 + 0.3284e-2 * t26396 + 0.15510666666666666667e2 * t26280 - 0.46531999999999999999e2 * t26284 - 0.38776666666666666665e1 * t26293 + 0.46532e2 * t26296 + 0.11633e2 * t26304 - 0.10340444444444444444e2 * t26311;
    let t27914 = 0.19704e-1 * t26406 - 0.14778e-1 * t26409 - 0.12315e-2 * t26412 + 0.29556e-1 * t26415 + 0.7389e-2 * t26419 - 0.12925555555555555555e2 * t26278 + 0.46531999999999999998e2 * t26289 - 0.69798e2 * t26300 - 0.15510666666666666667e2 * t26306 + 0.5170222222222222222e1 * t26309 + 0.14778e-1 * t26443 - 0.44334e-1 * t26446;
    (t27901, t27914)
}
