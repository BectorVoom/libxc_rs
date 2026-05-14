//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 921/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk921<F: Float>(t688: F, t9601: F, t677: F, t9682: F, t2395: F, t2428: F, t122: F, t196: F, t9606: F, t2380: F) -> (F, F, F, F, F) {
    let t41589 = t9601 * t688;
    let t41593 = t677 * t9682;
    let t41601 = t2395 * t2428;
    let t41621 = t122 / t196 / t9606;
    let t41622 = t2380 * t2380;
    (t41589, t41593, t41601, t41621, t41622)
}
