//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 823/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk823(t941: f64, t9576: f64, t3405: f64, t9575: f64, t3297: f64, t9552: f64, t2580: f64, t9166: f64, t2578: f64, t3284: f64, t7241: f64, t1092: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9577 = t941 * t9576;
    let t9578 = t3405 * t9577;
    let t9579 = t9575 * t9578;
    let t9581 = t9552 * t3297;
    let t9583 = t9166 * t2580;
    let t9584 = t2578 * t9583;
    let t9586 = t3284 * t7241;
    let t9587 = t1092 * t9586;
    (t9578, t9579, t9581, t9584, t9586, t9587)
}
