//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 708/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk708<F: Float>(t21362: F, t683: F, t92: F, t21204: F, t20489: F, t668: F) -> (F, F, F, F, F) {
    let t21363 = t683 * t21362;
    let t21364 = t92 * t21363;
    let t21366 = t683 * t21204;
    let t21367 = t92 * t21366;
    let t21369 = t668 * t20489;
    (t21363, t21364, t21366, t21367, t21369)
}
