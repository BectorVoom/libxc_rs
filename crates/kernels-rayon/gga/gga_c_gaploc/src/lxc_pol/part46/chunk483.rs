//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 483/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk483(t1: f64, t2754: f64, t106: f64, t192: f64, t1564: f64, t4529: f64, t986: f64, t2765: f64, t524: f64, t188: f64, t7930: f64, t493: f64, t7892: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8070 = t2754 * t1;
    let t8071 = t8070 * t106;
    let t8072 = t8071 * t192;
    let t8097 = t1564 * t2754;
    let t8124 = t4529 * t986;
    let t8155 = t524 * t2765;
    let t8158 = t188 * t7930;
    let t8195 = t493 * t7892;
    (t8072, t8097, t8124, t8155, t8158, t8195)
}
