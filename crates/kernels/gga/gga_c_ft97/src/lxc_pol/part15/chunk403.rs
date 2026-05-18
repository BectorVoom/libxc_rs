//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 403/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk403<F: Float>(t1026: F, t1882: F, t1047: F, t376: F, t89: F, t1039: F, t2086: F, t1033: F, t1775: F, t2: F, t2097: F, t582: F) -> (F, F, F, F, F, F) {
    let t3460 = t1882 * t1026;
    let t3489 = t89 * t376 * t1047;
    let t3491 = t2086 * t1039;
    let t3497 = t1775 * t1033;
    let t3499 = t2097 * t2;
    let t3506 = t582 * t2;
    (t3460, t3489, t3491, t3497, t3499, t3506)
}
