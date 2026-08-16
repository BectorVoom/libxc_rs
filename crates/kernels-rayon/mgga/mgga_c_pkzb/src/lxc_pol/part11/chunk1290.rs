//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1290/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1290(t3069: f64, t3769: f64, t6142: f64, t11264: f64, t18617: f64, t851: f64, t11261: f64, t2197: f64, t11260: f64, t2242: f64, t2240: f64, t6199: f64, t9867: f64) -> (f64, f64, f64, f64, f64) {
    let t31394 = 0.28947563097646563121e3_f64 * t6142 * t3769 * t3069;
    let t31397 = 0.62071215503128080361e4_f64 * t18617 * t11264 * t851;
    let t31400 = 2.0_f64 * t2197 * t11261 * t851;
    let t31401 = t11260 * t2242;
    let t31404 = 0.16081979498692535067e2_f64 * t2240 * t31401 * t851;
    let t31407 = 0.1551780387578202009e4_f64 * t6199 * t9867 * t3069;
    (t31394, t31397, t31400, t31404, t31407)
}
