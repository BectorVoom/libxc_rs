//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 215/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk215<F: Float>(t668: F, t703: F, t505: F, t420: F, t701: F, t699: F) -> (F, F, F, F, F) {
    let t704 = t703 * t668;
    let t705 = t704 * t505;
    let t706 = t420 * t705;
    let t707 = t701 * t706;
    let t709 = t699 + 0.6384360837962962963e-2 * t707;
    (t704, t705, t706, t707, t709)
}
