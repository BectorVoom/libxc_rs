//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1054/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1054<F: Float>(t27: F, t41751: F, t241: F, t41536: F, t41448: F, t89: F, t2336: F, t9703: F, t2345: F, t9717: F, t681: F, t9713: F) -> (F, F, F, F, F) {
    let t41911 = t27 * t41751;
    let t41912 = t241 * t41536;
    let t41915 = t89 * t41911 * t41912 * t41448;
    let t41918 = t89 * t2336 * t9703;
    let t41922 = t89 * t2345 * t9717 * t41448;
    let t41925 = t89 * t681 * t9713;
    (t41911, t41915, t41918, t41922, t41925)
}
