//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1998/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1998<F: Float>(t131: F, t845: F, t23143: F, t6649: F, t6604: F, t9971: F, t206: F, t22723: F, t268: F, t23186: F, t23163: F, t23165: F) -> (F, F, F, F, F, F, F) {
    let t81982 = t845 * t131;
    let t82011 = t23143 * t6649;
    let t82018 = t6604 * t9971;
    let t82031 = t22723 * t206 * t268;
    let t82032 = t82031 * t23186;
    let t82038 = t22723 * t23163;
    let t82039 = t82038 * t23165;
    (t81982, t82011, t82018, t82031, t82032, t82038, t82039)
}
