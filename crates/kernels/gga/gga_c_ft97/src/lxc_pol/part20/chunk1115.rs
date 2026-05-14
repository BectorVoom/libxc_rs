//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1115/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1115<F: Float>(t109357: F, t109359: F, t109361: F, t109367: F, t109372: F, t109375: F, t109379: F, t109382: F, t109385: F, t109388: F, t109393: F, t108097: F, t6118: F, t97078: F, t24546: F, t27819: F, t27820: F, t729: F) -> (F, F, F) {
    let t109395 = t109357 + t109359 + 11.0 / 9.0 * t109361 + 15.0 / 16.0 * t109367 + 3.0 / 4.0 * t109372 + t109375 / 9.0 + 5.0 / 27.0 * t109379 - 4.0 / 9.0 * t109382 - t109385 / 3.0 - 2.0 / 9.0 * t109388 + 3.0 * t109393;
    let t109397 = t6118 * t97078 * t108097;
    let t109400 = t27819 * t729 * t24546 * t27820;
    (t109395, t109397, t109400)
}
