//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 995/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk995<F: Float>(t8285: F, t92: F, t93: F, t352: F, t2663: F, t275: F, t2329: F, t2320: F, t2347: F, t287: F, t745: F, t355: F, t357: F, t362: F, t7329: F, t988: F) -> (F, F, F, F, F, F, F) {
    let t23518 = 1.0 / t8285 / t92 * t93;
    let t23519 = t23518 * t352;
    let t23520 = t2663 * t275;
    let t23523 = t2329 * t2329;
    let t23531 = t2320 * t2347;
    let t23533 = t745 * t287;
    let t23537 = 40.0 / 81.0 * t355 * t357 * t23533 * t362;
    let t23539 = t355 * t988 * t7329;
    (t23519, t23520, t23523, t23531, t23533, t23537, t23539)
}
