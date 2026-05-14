//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 800/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk800<F: Float>(t1033: F, t3139: F, t1037: F, t3051: F, t1045: F, t9438: F, t1055: F, t3281: F, t2101: F, t3578: F, t1026: F, t1047: F, t7943: F, t89: F, t10051: F, t1160: F) -> (F, F, F, F, F, F, F, F) {
    let t49782 = t3139 * t1033;
    let t49921 = t3051 * t1037;
    let t50260 = t1045 * t9438;
    let t50679 = t3281 * t1055;
    let t50773 = t2101 * t3578;
    let t50781 = t3281 * t1026;
    let t51149 = t89 * t7943 * t1047;
    let t51340 = t1160 * t10051;
    (t49782, t49921, t50260, t50679, t50773, t50781, t51149, t51340)
}
