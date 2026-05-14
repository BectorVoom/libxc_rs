//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 273/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk273<F: Float>(t1651: F, t378: F, t92: F, t1639: F, t1640: F, t1645: F, t1649: F) -> (F, F, F) {
    let t1652 = t378 * t1651;
    let t1653 = t92 * t1652;
    let t1655 = t1639 + 2.0 / 9.0 * t1640 - 2.0 / 9.0 * t1645 + 2.0 / 3.0 * t1649 - t1653 / 3.0;
    (t1652, t1653, t1655)
}
