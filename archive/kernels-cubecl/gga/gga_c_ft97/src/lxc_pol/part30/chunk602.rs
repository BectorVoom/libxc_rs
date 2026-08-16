//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 602/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk602<F: Float>(t24438: F, t27775: F, t6135: F, t24437: F, t27742: F, t676: F, t27: F, t89: F, t2: F, t6837: F, t2354: F, t684: F) -> (F, F, F, F) {
    let t27777 = t24438 * t6135 * t27775;
    let t27778 = t24437 * t27777;
    let t27781 = t676 * t27742;
    let t27783 = t89 * t27 * t27781;
    let t27787 = t2 * t6837;
    let t27789 = t2354 * t27787 * t684;
    (t27778, t27781, t27783, t27789)
}
