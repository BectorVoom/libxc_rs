//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 509/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk509<F: Float>(t1053: F, t558: F, t574: F, t605: F, t609: F, t2179: F, t144: F, t1047: F, t376: F, t89: F, t1039: F, t2086: F, t590: F, t91: F, t1033: F, t1775: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3478 = t1053 * t558;
    let t3480 = t574 * t605 * t3478;
    let t3483 = t1053 * t609;
    let t3484 = t2179 * t3483;
    let t3485 = t144 * t3484;
    let t3489 = t89 * t376 * t1047;
    let t3491 = t2086 * t1039;
    let t3493 = t91 * t3491 * t590;
    let t3497 = t1775 * t1033;
    (t3478, t3480, t3483, t3484, t3485, t3489, t3491, t3493, t3497)
}
