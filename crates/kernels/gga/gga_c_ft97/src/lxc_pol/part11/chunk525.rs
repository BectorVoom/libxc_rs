//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 525/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk525<F: Float>(t2: F, t2097: F, t582: F, t179: F, t422: F, t71: F, t1576: F, t171: F, t11: F, t41: F, t184: F, t21: F) -> (F, F, F, F, F, F, F) {
    let t3499 = t2097 * t2;
    let t3506 = t582 * t2;
    let t3613 = t422 * t179;
    let t3621 = t71 * t179;
    let t3626 = F::new(1.0) / t171 / t1576;
    let t3627 = t11 * t3626;
    let t3628 = t41 * t3627;
    let t3664 = t184 * t21;
    (t3499, t3506, t3613, t3621, t3626, t3628, t3664)
}
