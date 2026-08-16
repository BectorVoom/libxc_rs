//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1237/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1237(t10: f64, t128: f64, t14632: f64, t21: f64, t411: f64, t635: f64, t1652: f64, t763: f64, t1844: f64, t415: f64, t5594: f64, t156: f64, t1568: f64, t4: f64) -> (f64, f64, f64, f64, f64) {
    let t14634 = t10 * t128 * t14632;
    let t14639 = t21 * t635 * t411;
    let t14640 = t1652 * t763 * t14639;
    let t14641 = 1.9486833333333333_f64 * t14640;
    let t14643 = t415 * t1844 * t5594;
    let t14644 = 5.84605_f64 * t14643;
    let t14646 = t4 * t156 * t1568;
    (t14634, t14639, t14641, t14644, t14646)
}
