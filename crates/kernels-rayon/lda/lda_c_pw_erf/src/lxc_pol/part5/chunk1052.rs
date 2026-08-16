//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1052/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1052(t168: f64, t635: f64, t7025: f64, t1125: f64, t153: f64, t2357: f64, t632: f64, t7045: f64, t15483: f64, t242: f64, t7032: f64, t1143: f64, t2379: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19358 = t168 * t635 * t7025;
    let t19361 = t153 * t1125 * t2357;
    let t19363 = t7045 * t632;
    let t19365 = t15483 * t242;
    let t19385 = t7032 * t632;
    let t19388 = t2379 * t1143;
    (t19358, t19361, t19363, t19365, t19385, t19388)
}
