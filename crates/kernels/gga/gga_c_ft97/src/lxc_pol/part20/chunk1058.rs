//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1058/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1058<F: Float>(t108072: F, t108035: F, t108039: F, t108043: F, t108047: F, t108051: F, t108055: F, t108059: F, t108061: F, t108063: F, t108068: F, t108070: F, t1434: F, t27879: F, t681: F, t27846: F, t6109: F) -> (F, F, F, F) {
    let t108073 = 2.0 / 9.0 * t108072;
    let t108074 = 2.0 / 9.0 * t108035 - t108039 / 6.0 + t108043 / 6.0 + 2.0 / 3.0 * t108047 - 2.0 / 3.0 * t108051 - t108055 / 6.0 + t108059 + t108061 - 4.0 / 3.0 * t108063 + t108068 / 4.0 + 22.0 / 9.0 * t108070 - t108073;
    let t108077 = t1434 * t681 * t27879;
    let t108078 = 2.0 / 3.0 * t108077;
    let t108080 = t6109 * t681 * t27846;
    (t108074, t108077, t108078, t108080)
}
