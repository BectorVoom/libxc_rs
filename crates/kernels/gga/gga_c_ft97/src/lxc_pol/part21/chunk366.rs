//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 366/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk366<F: Float>(t2179: F, t3483: F, t144: F, t1047: F, t376: F, t89: F, t1039: F, t2086: F, t590: F, t91: F, t1033: F, t1775: F, t2: F, t2097: F, t2984: F, t2102: F, t3323: F) -> (F, F, F, F, F, F, F, F) {
    let t3484 = t2179 * t3483;
    let t3485 = t144 * t3484;
    let t3489 = t89 * t376 * t1047;
    let t3491 = t2086 * t1039;
    let t3493 = t91 * t3491 * t590;
    let t3497 = t1775 * t1033;
    let t3499 = t2097 * t2;
    let t3500 = t3499 * t2984;
    let t3503 = t2102 * t3323;
    (t3485, t3489, t3491, t3493, t3497, t3499, t3500, t3503)
}
