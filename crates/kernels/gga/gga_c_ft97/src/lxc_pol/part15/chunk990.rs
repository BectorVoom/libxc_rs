//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 990/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk990<F: Float>(t22441: F, t681: F, t89: F, t1882: F, t22218: F, t22222: F, t22183: F, t8392: F, t22188: F, t22402: F, t22261: F, t22373: F) -> (F, F, F, F, F, F, F, F) {
    let t84625 = t89 * t681 * t22441;
    let t84628 = t1882 * t22218;
    let t84630 = t1882 * t22222;
    let t84697 = t8392 * t22183;
    let t84734 = t8392 * t22188;
    let t84740 = t1882 * t22402;
    let t84767 = t1882 * t22261;
    let t84795 = t8392 * t22373;
    (t84625, t84628, t84630, t84697, t84734, t84740, t84767, t84795)
}
