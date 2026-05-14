//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 564/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk564<F: Float>(t2404: F, t675: F, t241: F, t9577: F, t2371: F, t683: F, t2360: F, t761: F, t2344: F) -> (F, F, F, F, F) {
    let t9744 = t2404 * t675;
    let t9749 = t241 * t9577;
    let t9770 = t683 * t2371;
    let t9791 = t761 * t2360;
    let t9802 = t2344 * t675;
    (t9744, t9749, t9770, t9791, t9802)
}
