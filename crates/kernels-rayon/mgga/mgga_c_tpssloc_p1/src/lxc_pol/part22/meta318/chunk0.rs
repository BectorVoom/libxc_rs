//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1500/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1500(t11697: f64, t4949: f64, t3577: f64, t3431: f64, t4729: f64, t1174: f64, t1011: f64, t15031: f64, t1212: f64, t1226: f64, t4965: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15572 = t11697 * t4949;
    let t15574 = t3577 * t15572 / 3456.0_f64;
    let t15578 = t3431 * t4729;
    let t15580 = t1174 * t15578 / 216.0_f64;
    let t15590 = t15031 * t1011;
    let t15591 = t15590 * t1212;
    let t15594 = t4965 * t1226;
    (t15572, t15574, t15578, t15580, t15590, t15591, t15594)
}
