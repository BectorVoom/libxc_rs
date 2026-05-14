//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 612/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk612<F: Float>(t3936: F, t458: F, t1775: F, t3927: F, t1609: F, t2378: F) -> (F, F, F) {
    let t13345 = 2.0 / 3.0 * t458 * t3936;
    let t13388 = 2.0 / 9.0 * t1775 * t3927;
    let t13411 = t1609 * t2378;
    (t13345, t13388, t13411)
}
