//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 353/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk353<F: Float>(t11: F, t5589: F, t14: F, t53: F, t72: F, t1710: F, t6: F, t8: F, t3076: F) -> (F, F, F, F, F, F, F) {
    let t5590 = t11 * t5589;
    let t5591 = t5590 * t14;
    let t5592 = t72 * t53;
    let t5593 = t5591 * t5592;
    let t5596 = t1710 * t6;
    let t5597 = t5596 * t8;
    let t5598 = t3076 * t5597;
    (t5590, t5591, t5592, t5593, t5596, t5597, t5598)
}
