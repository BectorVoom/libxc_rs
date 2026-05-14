//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 309/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk309<F: Float>(t2: F, t582: F, t2993: F, t3140: F, t1037: F, t458: F, t2102: F, t3338: F, t1017: F, t1985: F, t558: F, t24: F, t3408: F, t586: F, t2092: F, t2093: F, t2095: F, t3139: F, t3497: F, t3500: F, t3503: F, t462: F, t92: F) -> (F,) {
    let t3506 = t582 * t2;
    let t3507 = t3506 * t2993;
    let t3510 = t582 * t3140;
    let t3513 = t458 * t1037;
    let t3515 = t2102 * t3338;
    let t3518 = t2 * t1017;
    let t3520 = t1985 * t3518 * t558;
    let t3524 = t24 * t586 * t3408;
    let t3526 = t2092 + t2093 / 9.0 + t2095 / 3.0 + t3497 / 9.0 - 2.0 / 9.0 * t462 * t3500 + t462 * t3503 / 3.0 + 2.0 / 3.0 * t462 * t3507 - 2.0 / 3.0 * t3139 * t3510 + t3513 / 3.0 + t462 * t3515 / 3.0 + 2.0 * t462 * t3520 - t92 * t3524;
    (t3526,)
}
