//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 773/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk773<F: Float>(t21355: F, t2404: F, t92: F, t21196: F, t21181: F, t2347: F) -> (F, F, F, F, F) {
    let t21356 = t2404 * t21355;
    let t21357 = t92 * t21356;
    let t21359 = t2404 * t21196;
    let t21360 = t92 * t21359;
    let t21362 = t2347 * t21181;
    (t21356, t21357, t21359, t21360, t21362)
}
