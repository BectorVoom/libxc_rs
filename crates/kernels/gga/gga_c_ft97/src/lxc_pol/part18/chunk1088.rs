//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1088/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1088<F: Float>(t1624: F, t9: F, t92461: F, t173: F, t22584: F, t22583: F, t22586: F, t1602: F, t1711: F, t22582: F, t1630: F, t420: F, t422: F, t1742: F, t409: F, t5532: F) -> (F, F, F, F, F, F, F) {
    let t92463 = t1624 * t9 * t92461;
    let t92466 = t173 * t22584;
    let t92468 = t22583 * t92466 * t22586;
    let t92470 = t1602 * t1711;
    let t92471 = t92470 * t22582;
    let t92476 = t420 * t422 * t1630;
    let t92482 = t420 * t1742;
    let t92488 = t409 * t5532;
    (t92463, t92466, t92468, t92471, t92476, t92482, t92488)
}
