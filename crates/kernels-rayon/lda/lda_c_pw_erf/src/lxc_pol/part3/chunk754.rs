//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 754/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk754(t219: f64, t4867: f64, t4676: f64, t571: f64, t2021: f64, t954: f64, t1308: f64, t2193: f64, t3416: f64, t1450: f64, t2171: f64, t2098: f64, t529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4868 = t4867 * t219;
    let t4869 = t4868 * t4676;
    let t4871 = 16.0_f64 / 27.0_f64 * t571 * t4869;
    let t4872 = t2021 * t954;
    let t4873 = t1308 * t4872;
    let t4875 = 4.0_f64 / 45.0_f64 * t571 * t4873;
    let t4877 = 8.0_f64 / 15.0_f64 * t3416 * t2193;
    let t4879 = 16.0_f64 / 135.0_f64 * t2171 * t1450;
    let t4880 = t529 * t2098;
    (t4868, t4869, t4871, t4872, t4873, t4875, t4877, t4879, t4880)
}
