//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 200/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk200<F: Float>(t383: F, t528: F, t120: F, t30: F, t31: F, t123: F) -> (F, F, F, F, F) {
    let t529 = t528 * t383;
    let t530 = t529 * t120;
    let t532 = t31 * t30;
    let t533 = 1.0 / t532;
    let t534 = t123 * t533;
    (t529, t530, t532, t533, t534)
}
