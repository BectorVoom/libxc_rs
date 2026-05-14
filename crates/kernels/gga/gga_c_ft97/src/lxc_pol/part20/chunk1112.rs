//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1112/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1112<F: Float>(t2354: F, t2409: F, t27787: F, t6118: F, t1091: F, t97005: F, t1154: F, t24437: F, t2459: F, t2574: F, t6119: F, t108437: F, t108441: F, t109325: F, t109329: F, t109333: F, t109337: F, t97367: F, t97369: F, t97370: F) -> (F, F, F, F) {
    let t109341 = t6118 * t2354 * t27787 * t2409;
    let t109345 = t6118 * t2354 * t97005 * t1091;
    let t109350 = t24437 * t2574 * t6119 * t1154 * t2459;
    let t109352 = -4.0 / 3.0 * t108437 - 2.0 / 3.0 * t108441 - t109325 / 2.0 + 24.0 * t109329 + t97367 - 2.0 / 3.0 * t109333 + 4.0 * t109337 - t109341 / 3.0 - t97369 - t97370 + t109345 / 6.0 - t109350 / 2.0;
    (t109341, t109345, t109350, t109352)
}
