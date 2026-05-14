//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 225/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk225<F: Float>(t140: F, t1291: F, t1303: F, t1355: F, t543: F) -> (F,) {
    let t141 = 0.1e-59 < t140;
    let t1359 = piecewise3(t141, 0.22653425206514361674e0 * t543 * t1291 - 0.22653425206514361674e0 * t140 * t1291 - 0.50008500819444444447e-1 * t1355 * t1303, 0.0);
    (t1359,)
}
