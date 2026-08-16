//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1041/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1041(t330: f64, t8591: f64, t328: f64, t1029: f64, t133: f64, t1044: f64, t1717: f64, t588: f64, t1123: f64, t2003: f64, t300: f64, t5955: f64, t759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8592 = t330 * t8591;
    let t8593 = t328 * t8592;
    let t8865 = t1029 * t133;
    let t9056 = t1717 * t1044;
    let t9067 = t588 * t1044;
    let t9257 = t2003 * t1123;
    let t9258 = t300 * t9257;
    let t9319 = t5955 * t759;
    (t8593, t8865, t9056, t9067, t9258, t9319)
}
