//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1091/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1091<F: Float>(t100800: F, t5838: F, t26604: F, t26762: F, t22632: F, t26607: F, t5813: F, t1008: F, t5555: F, t12411: F, t1354: F, t6604: F, t92557: F, t92433: F, t104689: F, t2001: F) -> (F, F, F, F, F, F, F, F) {
    let t105058 = t5838 * t100800;
    let t105124 = t26604 * t26762;
    let t105127 = t5813 * t22632 * t26607;
    let t105157 = t5555 * t1008;
    let t105201 = t12411 * t1354;
    let t105207 = t5813 * t92557 * t6604;
    let t105211 = 0.17780800291358024692e0 * t5813 * t92433 * t6604;
    let t105212 = t2001 * t104689;
    (t105058, t105124, t105127, t105157, t105201, t105207, t105211, t105212)
}
