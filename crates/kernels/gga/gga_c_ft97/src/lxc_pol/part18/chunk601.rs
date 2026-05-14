//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 601/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk601<F: Float>(t11: F, t2247: F, t41: F, t634: F, t2253: F, t2277: F, t2261: F, t2284: F, t422: F, t639: F, t2252: F, t70: F) -> (F, F, F, F, F, F, F, F) {
    let t8639 = t11 * t2247;
    let t8640 = t41 * t8639;
    let t8641 = t8640 * t634;
    let t8643 = t2253 * t2277;
    let t8645 = t2253 * t2261;
    let t8647 = t2253 * t2284;
    let t8654 = t422 * t639;
    let t8675 = t41 * t2252 * t70;
    (t8639, t8640, t8641, t8643, t8645, t8647, t8654, t8675)
}
