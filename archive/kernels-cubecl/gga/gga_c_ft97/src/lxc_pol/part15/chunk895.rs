//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 895/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk895<F: Float>(t1045: F, t9114: F, t1030: F, t3281: F, t1033: F, t3139: F, t1037: F, t3051: F, t9438: F, t1055: F, t2101: F, t3578: F) -> (F, F, F, F, F, F, F) {
    let t49634 = t9114 * t1045;
    let t49661 = t3281 * t1030;
    let t49782 = t3139 * t1033;
    let t49921 = t3051 * t1037;
    let t50260 = t1045 * t9438;
    let t50679 = t3281 * t1055;
    let t50773 = t2101 * t3578;
    (t49634, t49661, t49782, t49921, t50260, t50679, t50773)
}
