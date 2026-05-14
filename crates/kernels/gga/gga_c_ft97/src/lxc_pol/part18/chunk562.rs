//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 562/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk562<F: Float>(t574: F, t605: F, t6699: F, t1384: F, t3578: F, t144: F, t1053: F) -> (F, F, F, F) {
    let t6701 = t574 * t605 * t6699;
    let t6704 = t3578 * t1384;
    let t6705 = t144 * t6704;
    let t6708 = t1384 * t1053;
    (t6701, t6704, t6705, t6708)
}
