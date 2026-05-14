//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 305/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk305<F: Float>(t379: F, t432: F, t1564: F, t446: F, t21: F, t357: F) -> (F, F, F, F) {
    let t1565 = t379 * t432;
    let t1566 = t1564 * t1565;
    let t1567 = t446 * t1566;
    let t1569 = t357 * t21;
    let t1570 = 1.0 / t1569;
    (t1565, t1566, t1567, t1570)
}
