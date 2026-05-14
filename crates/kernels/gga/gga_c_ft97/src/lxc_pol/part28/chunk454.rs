//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 454/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk454<F: Float>(t1374: F, t461: F, t1359: F, t72: F, t2: F, t342: F, t343: F, t7298: F, t4: F, t26: F) -> (F, F, F, F) {
    let t7299 = t461 * t1374;
    let t7302 = t72 * t1359;
    let t7307 = (-t7298 * t7299 / 6.0 - t342 * t343 * t7302 / 4.0) * t2;
    let t7308 = t7307 * t4;
    let t7309 = t7308 * t26;
    (t7299, t7302, t7308, t7309)
}
