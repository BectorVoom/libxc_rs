//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 340/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk340<F: Float>(t128: F, t131: F, t135: F, t527: F, t118: F, t29: F, t341: F, t343: F) -> (F, F, F, F) {
    let t1995 = t128 * t131;
    let t2001 = t527 * t135;
    let t2007 = 1.0 / t118 / t29;
    let t2014 = t341 * t343;
    (t1995, t2001, t2007, t2014)
}
