//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 564/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk564<F: Float>(t5632: F, t8392: F, t1882: F, t5712: F, t5661: F, t1359: F, t7368: F, t1348: F, t458: F) -> (F, F, F, F, F) {
    let t23344 = t8392 * t5632;
    let t23358 = t1882 * t5712;
    let t23360 = t1882 * t5661;
    let t23400 = t7368 * t1359;
    let t23405 = t1348 * t458;
    (t23344, t23358, t23360, t23400, t23405)
}
