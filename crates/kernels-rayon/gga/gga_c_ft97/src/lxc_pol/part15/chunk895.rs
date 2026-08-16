//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 895/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk895(t1045: f64, t9114: f64, t1030: f64, t3281: f64, t1033: f64, t3139: f64, t1037: f64, t3051: f64, t9438: f64, t1055: f64, t2101: f64, t3578: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49634 = t9114 * t1045;
    let t49661 = t3281 * t1030;
    let t49782 = t3139 * t1033;
    let t49921 = t3051 * t1037;
    let t50260 = t1045 * t9438;
    let t50679 = t3281 * t1055;
    let t50773 = t2101 * t3578;
    (t49634, t49661, t49782, t49921, t50260, t50679, t50773)
}
