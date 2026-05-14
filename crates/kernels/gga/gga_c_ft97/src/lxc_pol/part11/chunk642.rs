//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 642/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk642<F: Float>(t718: F, t9511: F, t2330: F, t2464: F, t2460: F, t375: F, t89: F, t194: F, t196: F, t122: F) -> (F, F, F, F) {
    let t9512 = t9511 * t718;
    let t9514 = t2330 * t2464;
    let t9520 = t89 * t375 * t2460;
    let t9523 = 1.0 / t196 / t194;
    let t9524 = t122 * t9523;
    (t9512, t9514, t9520, t9524)
}
