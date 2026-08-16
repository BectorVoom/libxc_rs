//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 741/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk741(t1: f64, t25760: f64, t20550: f64, t7892: f64, t7905: f64, t9448: f64, t9439: f64, t1415: f64, t8247: f64, t4348: f64, t997: f64, t1033: f64, t5558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26126 = t25760 * t1;
    let t26328 = t20550 * t7892;
    let t26435 = t9448 * t7905;
    let t26922 = t9439 * t7905;
    let t26984 = t1415 * t8247;
    let t27003 = t9439 * t7892;
    let t27007 = t9448 * t7892;
    let t27214 = t997 * t4348;
    let t27229 = t1033 * t5558;
    (t26126, t26328, t26435, t26922, t26984, t27003, t27007, t27214, t27229)
}
