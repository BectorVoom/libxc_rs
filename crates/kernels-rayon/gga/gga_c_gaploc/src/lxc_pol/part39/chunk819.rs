//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 819/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk819(t7905: f64, t9448: f64, t10555: f64, t107: f64, t544: f64, t2754: f64, t4529: f64, t9439: f64, t524: f64, t7937: f64, t1570: f64, t188: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26435 = t9448 * t7905;
    let t26796 = t544 * t10555 * t107;
    let t26809 = t4529 * t2754;
    let t26922 = t9439 * t7905;
    let t26935 = t524 * t7937;
    let t26938 = t1570 * t2754;
    let t26939 = t188 * t26938;
    (t26435, t26796, t26809, t26922, t26935, t26938, t26939)
}
