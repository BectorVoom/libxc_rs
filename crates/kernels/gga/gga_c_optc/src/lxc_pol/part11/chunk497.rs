//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 497/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk497<F: Float>(t3313: F, t3315: F, t115: F, t56: F, t5: F, t1261: F, t2007: F, t1235: F, t1933: F, t1239: F, t1940: F, t1271: F, t2024: F) -> (F, F, F, F, F, F) {
    let t3316 = t3313 * t3315;
    let t3317 = t56 * t115;
    let t3318 = t3317 * t5;
    let t3325 = t2007 * t1261;
    let t3331 = t1933 * t1235;
    let t3339 = t1940 * t1239;
    let t3353 = t1271 * t2024;
    (t3316, t3318, t3325, t3331, t3339, t3353)
}
