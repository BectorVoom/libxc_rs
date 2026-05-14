//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 985/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk985<F: Float>(t29285: F, t29287: F, t29290: F, t29295: F, t29299: F, t29304: F, t29309: F, t29313: F, t29317: F, t29321: F, t29325: F, t29329: F, t29332: F, t29334: F, t446: F, t1882: F, t7098: F) -> (F, F) {
    let t29336 = t29285 / 9.0 - 2.0 / 9.0 * t29287 + 2.0 / 3.0 * t446 * t29290 + t446 * t29295 / 3.0 + t446 * t29299 / 3.0 + t446 * t29304 / 3.0 + t446 * t29309 / 3.0 + 2.0 / 3.0 * t446 * t29313 + t446 * t29317 / 3.0 + 2.0 / 3.0 * t446 * t29321 + 2.0 / 3.0 * t446 * t29325 + 2.0 / 3.0 * t446 * t29329 - 2.0 / 9.0 * t29332 - t29334 / 9.0;
    let t29340 = t1882 * t7098;
    (t29336, t29340)
}
