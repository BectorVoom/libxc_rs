//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2455/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2455(t11545: f64, t241: f64, t3241: f64, t242: f64, t281: f64, t415: f64, t2394: f64, t3253: f64) -> (f64, f64, f64, f64, f64) {
    let t43761 = t241 * t11545;
    let t43762 = t3241 * t3241;
    let t43763 = 1.0_f64 / t43762;
    let t43776 = t281 * t242 * t415;
    let t43777 = 0.13490888888888888889e1_f64 * t43776;
    let t43780 = t2394 * t3253;
    (t43761, t43763, t43776, t43777, t43780)
}
