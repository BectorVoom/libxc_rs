//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1084/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1084<F: Float>(t2441: F, t420: F, t3762: F, t3886: F, t27637: F, t684: F, t3758: F, t695: F, t24275: F, t27652: F, t1613: F, t6817: F, t92354: F, t2387: F, t27660: F, t200: F, t668: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108503 = t420 * t2441;
    let t108504 = t3886 * t3762;
    let t108508 = t420 * t27637;
    let t108509 = t684 * t3762;
    let t108517 = t3758 * t695;
    let t108518 = t108517 * t24275;
    let t108519 = t420 * t27652;
    let t108524 = t92354 * t1613 * t6817;
    let t108525 = t2387 * t108524;
    let t108526 = t420 * t27660;
    let t108530 = t200 * t668;
    (t108503, t108504, t108508, t108509, t108518, t108519, t108524, t108525, t108526, t108530)
}
