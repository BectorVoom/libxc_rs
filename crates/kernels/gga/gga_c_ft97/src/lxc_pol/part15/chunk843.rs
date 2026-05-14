//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 843/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk843<F: Float>(t1882: F, t20884: F, t20939: F, t20661: F, t375: F, t89: F, t1546: F, t20538: F, t20534: F, t37401: F, t20656: F, t20542: F, t20546: F, t20667: F, t20549: F, t7780: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77823 = t1882 * t20884;
    let t77868 = t1882 * t20939;
    let t77914 = t89 * t375 * t20661;
    let t77917 = t89 * t1546 * t20538;
    let t77920 = t89 * t37401 * t20534;
    let t77935 = t89 * t375 * t20656;
    let t77990 = t1882 * t20542;
    let t78001 = t1882 * t20546;
    let t78012 = t89 * t1546 * t20667;
    let t78015 = t89 * t7780 * t20549;
    (t77823, t77868, t77914, t77917, t77920, t77935, t77990, t78001, t78012, t78015)
}
