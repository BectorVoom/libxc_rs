//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1121/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1121<F: Float>(t24372: F, t27561: F, t697: F, t24360: F, t3766: F, t27529: F, t27609: F, t27557: F, t6034: F, t2441: F, t420: F, t27637: F, t3758: F, t695: F, t24275: F, t27652: F) -> (F, F, F, F, F, F, F, F, F) {
    let t108454 = t24372 * t697 * t27561;
    let t108487 = t3766 * t24360;
    let t108494 = 0.29693535778629056444e-3 * t27609 * t697 * t27529;
    let t108501 = 0.29693535778629056444e-4 * t6034 * t697 * t27557;
    let t108503 = t420 * t2441;
    let t108508 = t420 * t27637;
    let t108517 = t3758 * t695;
    let t108518 = t108517 * t24275;
    let t108519 = t420 * t27652;
    (t108454, t108487, t108494, t108501, t108503, t108508, t108517, t108518, t108519)
}
