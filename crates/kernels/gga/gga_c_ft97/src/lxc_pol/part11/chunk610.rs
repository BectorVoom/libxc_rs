//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 610/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk610<F: Float>(t526: F, t9007: F, t27: F, t89: F, t519: F, t7745: F, t356: F, t23: F, t7368: F, t1986: F, t558: F) -> (F, F, F, F, F, F) {
    let t9008 = t526 * t9007;
    let t9010 = t89 * t27 * t9008;
    let t9012 = t519 * t7745;
    let t9014 = t89 * t356 * t9012;
    let t9016 = t23 * t7368;
    let t9017 = t1986 * t558;
    (t9008, t9010, t9012, t9014, t9016, t9017)
}
