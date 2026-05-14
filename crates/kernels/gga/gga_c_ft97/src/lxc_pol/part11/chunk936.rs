//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 936/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk936<F: Float>(t2362: F, t89: F, t9733: F, t2336: F, t9737: F, t41448: F, t666: F, t9749: F, t2361: F, t41468: F, t27: F, t41751: F, t241: F, t41536: F, t9703: F, t2345: F, t9717: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41898 = t89 * t9733 * t2362;
    let t41899 = 8.0 / 27.0 * t41898;
    let t41901 = t89 * t2336 * t9737;
    let t41905 = t89 * t666 * t9749 * t41448;
    let t41909 = t89 * t666 * t2361 * t41468;
    let t41911 = t27 * t41751;
    let t41912 = t241 * t41536;
    let t41915 = t89 * t41911 * t41912 * t41448;
    let t41918 = t89 * t2336 * t9703;
    let t41922 = t89 * t2345 * t9717 * t41448;
    (t41898, t41899, t41901, t41905, t41909, t41911, t41915, t41918, t41922)
}
