//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 215/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk215<F: Float>(t172: F, t209: F, t231: F, t228: F, t227: F, t9: F) -> (F, F, F) {
    let t696 = t209 * t172;
    let t697 = t696 * t231;
    let t698 = t228 * t697;
    let t699 = 0.6384360837962962963e-2 * t698;
    let t701 = t9 * t227 * t209;
    (t698, t699, t701)
}
