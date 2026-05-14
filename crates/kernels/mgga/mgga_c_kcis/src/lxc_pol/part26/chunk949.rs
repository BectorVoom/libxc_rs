//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 949/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk949<F: Float>(t125: F, t26446: F, t86: F, t2398: F, t2720: F, t2157: F, t137: F, t2425: F, t2421: F, t695: F, t8939: F, t68: F) -> (F, F, F, F, F, F, F, F) {
    let t26448 = t86 * t125 * t26446;
    let t26450 = t2720 * t2398;
    let t26451 = t26450 * t2157;
    let t26454 = t86 * t2425 * t137;
    let t26457 = t86 * t2421 * t137;
    let t26459 = t8939 * t695;
    let t26460 = t26459 * t2157;
    let t26462 = t695 * t68;
    (t26448, t26450, t26451, t26454, t26457, t26459, t26460, t26462)
}
