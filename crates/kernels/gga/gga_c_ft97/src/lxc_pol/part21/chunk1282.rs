//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1282/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1282<F: Float>(t3408: F, t446: F, t6630: F, t9432: F, t105570: F, t105598: F, t105617: F, t105638: F, t105672: F, t119906: F, t119913: F, t119917: F, t119922: F, t119926: F, t96083: F, t30274: F, t376: F, t89: F) -> (F, F, F) {
    let t119930 = t446 * t9432 * t6630 * t3408;
    let t119932 = -2.0 / 3.0 * t119906 + 16.0 / 9.0 * t105570 + 4.0 / 27.0 * t105598 - t119913 - t119917 + 8.0 / 9.0 * t105617 + t96083 - t105638 + t119922 / 3.0 + 4.0 / 3.0 * t119926 - 12.0 * t119930 - t105672;
    let t119935 = t89 * t376 * t30274;
    (t119930, t119932, t119935)
}
