//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1118/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1118<F: Float>(t1636: F, t6903: F, t89: F, t27781: F, t375: F, t109431: F, t97220: F, t97232: F, t97244: F, t97248: F, t97399: F, t97400: F, t97403: F, t97408: F, t97409: F, t1434: F, t27743: F, t681: F) -> (F, F, F, F) {
    let t109434 = t89 * t1636 * t6903;
    let t109435 = 4.0 / 9.0 * t109434;
    let t109437 = t89 * t375 * t27781;
    let t109438 = 2.0 / 3.0 * t109437;
    let t109440 = -t97399 + t97400 + t97403 - t109431 + t97220 + 2.0 / 9.0 * t97232 - t97408 - t97409 - t109435 + t109438 - 8.0 / 9.0 * t97244 + t97248;
    let t109442 = t1434 * t681 * t27743;
    (t109434, t109437, t109440, t109442)
}
