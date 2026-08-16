//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 932/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk932(t1490: f64, t9836: f64, t1435: f64, t2637: f64, t131: f64, t2640: f64, t1348: f64, t1215: f64, t2625: f64, t372: f64, t7766: f64, t1349: f64) -> (f64, f64, f64, f64, f64) {
    let t9865 = t1490 * t9836;
    let t9867 = t2637 * t1435;
    let t9869 = t131 * t2640;
    let t9870 = t1348 * t9869;
    let t9874 = t2625 * t1215;
    let t9877 = t372 * t7766;
    let t9878 = t1349 * t9877;
    (t9865, t9867, t9870, t9874, t9878)
}
