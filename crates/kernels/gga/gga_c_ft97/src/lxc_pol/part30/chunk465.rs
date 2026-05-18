//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 465/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk465<F: Float>(t7440: F, t7515: F, t7511: F, t7512: F, t2506: F, t1434: F, t193: F, t743: F, t7484: F, t2372: F, t27: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t7516 = t7515 * t7440;
    let t7518 = t7511 * t7512 * t7516;
    let t7520 = t2506 * t7440;
    let t7522 = t1434 * t193 * t7520;
    let t7524 = t743 * t7484;
    let t7526 = t1434 * t193 * t7524;
    let t7528 = t2372 * t7440;
    let t7530 = t89 * t27 * t7528;
    (t7516, t7518, t7520, t7522, t7524, t7526, t7528, t7530)
}
