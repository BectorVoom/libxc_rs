//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 452/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk452<F: Float>(t488: F, t7281: F, t83: F, t28: F, t446: F, t7222: F, t7226: F, t7231: F, t7235: F, t7266: F, t7271: F, t7276: F, t89: F, t103: F, t7264: F) -> (F, F, F, F) {
    let t7282 = t488 * t7281;
    let t7283 = t83 * t7282;
    let t7286 = 2.0 / 3.0 * t446 * t7222 - 2.0 / 3.0 * t446 * t7226 + 2.0 / 3.0 * t446 * t7231 - t446 * t7235 / 3.0 + t89 * t28 * t7266 / 3.0 - 2.0 / 3.0 * t446 * t7271 + 2.0 / 3.0 * t446 * t7276 - t446 * t7283 / 3.0;
    let t7288 = t7264 * t103;
    (t7282, t7283, t7286, t7288)
}
