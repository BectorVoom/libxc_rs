//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 543/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk543<F: Float>(t1702: F, t52: F, t1614: F, t408: F, t1608: F, t373: F, t1593: F, t1609: F) -> (F, F, F) {
    let t7839 = t52 * t1702;
    let t7843 = t408 * t1614;
    let t7845 = t1608 * t7843 * t373;
    let t7857 = t1609 * t1593;
    (t7839, t7845, t7857)
}
