//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 914/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk914<F: Float>(t10409: F, t28524: F, t446: F, t1486: F, t681: F, t7075: F, t1882: F, t7080: F, t668: F, t7021: F) -> (F, F, F, F, F) {
    let t28525 = t10409 * t28524;
    let t28526 = t446 * t28525;
    let t28529 = t1486 * t681 * t7075;
    let t28531 = t1882 * t7080;
    let t28533 = t7021 * t668;
    (t28525, t28526, t28529, t28531, t28533)
}
