//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1449/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1449(t3475: f64, t4899: f64, t11545: f64, t135: f64, t11548: f64, t1174: f64, t43791: f64, t461: f64, t3439: f64, t698: f64, t3442: f64, t11588: f64) -> (f64, f64, f64, f64, f64) {
    let t44558 = t4899 * t3475;
    let t44562 = t135 * t11545;
    let t44564 = t1174 * t44562 * t11548;
    let t44566 = t461 * t43791;
    let t44571 = t698 * t3439;
    let t44573 = t1174 * t44571 * t3442;
    let t44579 = t11588 * t3475;
    (t44558, t44564, t44566, t44573, t44579)
}
