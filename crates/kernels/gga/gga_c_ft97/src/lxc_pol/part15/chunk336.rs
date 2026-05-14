//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 336/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk336<F: Float>(t240: F, t668: F, t10: F, t1542: F, t242: F, t375: F, t665: F) -> (F, F, F, F) {
    let t2321 = t240 * t668;
    let t2334 = t10 * t1542 * t242;
    let t2335 = 2.0 / 27.0 * t2334;
    let t2336 = t375 * t665;
    (t2321, t2334, t2335, t2336)
}
