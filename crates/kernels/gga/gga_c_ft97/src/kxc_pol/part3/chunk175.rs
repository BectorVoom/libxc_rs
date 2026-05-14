//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 175/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk175<F: Float>(t139: F, t538: F, t527: F, t129: F, t131: F, t137: F) -> (F, F, F, F, F) {
    let t539 = t139 * t538;
    let t540 = t527 * t539;
    let t542 = t129 * t131;
    let t543 = t542 * t139;
    let t548 = t137 * t137;
    let t549 = 1.0 / t548;
    (t540, t542, t543, t548, t549)
}
