//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 761/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk761<F: Float>(t21196: F, t3806: F, t701: F, t21181: F, t9665: F, t420: F, t3699: F, t4635: F) -> (F, F, F, F, F, F) {
    let t21197 = t3806 * t21196;
    let t21198 = t701 * t21197;
    let t21200 = t9665 * t21181;
    let t21201 = t420 * t21200;
    let t21202 = t701 * t21201;
    let t21204 = t3699 * t4635;
    (t21197, t21198, t21200, t21201, t21202, t21204)
}
