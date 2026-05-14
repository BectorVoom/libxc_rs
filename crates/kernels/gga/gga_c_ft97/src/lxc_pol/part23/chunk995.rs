//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 995/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk995<F: Float>(t2393: F, t4939: F, t200: F, t6014: F, t24373: F, t4960: F, t1091: F, t27637: F, t24269: F, t224: F, t695: F, t206: F, t4999: F, t5011: F) -> (F, F, F, F, F, F, F, F) {
    let t30651 = t2393 * t4939;
    let t30652 = t30651 * t200;
    let t30653 = t6014 * t30652;
    let t30656 = t24373 * t4960;
    let t30660 = t27637 * t1091;
    let t30667 = t24269 * t4960;
    let t30671 = t224 * t695;
    let t30674 = 1.0 / t206 / t5011 / t4999;
    (t30651, t30652, t30653, t30656, t30660, t30667, t30671, t30674)
}
