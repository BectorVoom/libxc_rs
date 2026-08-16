//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3200/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3200(t11697: f64, t18968: f64, t3577: f64, t11539: f64, t1174: f64, t18232: f64, t18215: f64, t1734: f64, t3584: f64, t375: f64, t11665: f64, t18371: f64) -> (f64, f64, f64, f64, f64) {
    let t66566 = t3577 * t11697 * t18968;
    let t66571 = t1174 * t11539 * t18232;
    let t66575 = t1174 * t11539 * t18215;
    let t66583 = t375 * t3584 * t1734;
    let t66597 = t11665 * t18371;
    (t66566, t66571, t66575, t66583, t66597)
}
