//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 969/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk969(t1882: f64, t21689: f64, t21717: f64, t761: f64, t21439: f64, t21405: f64, t21417: f64, t375: f64, t89: f64, t21399: f64, t668: f64, t21409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80477 = t1882 * t21689;
    let t80522 = t21717 * t761;
    let t80677 = t1882 * t21439;
    let t80679 = t1882 * t21405;
    let t80685 = t89 * t375 * t21417;
    let t80691 = t21399 * t668;
    let t80696 = t1882 * t21409;
    (t80477, t80522, t80677, t80679, t80685, t80691, t80696)
}
