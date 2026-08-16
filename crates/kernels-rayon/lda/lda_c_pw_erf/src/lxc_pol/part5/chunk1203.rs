//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1203/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1203(t17788: f64, t184: f64, t2423: f64, t549: f64, t813: f64, t17794: f64, t352: f64, t7414: f64, t11: f64, t557: f64, t1953: f64, t21219: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21771 = 16.0_f64 / 15.0_f64 * t17788;
    let t21775 = 4.0_f64 / 5.0_f64 * t549 * t2423 * t184 * t813;
    let t21776 = 8.0_f64 / 45.0_f64 * t17794;
    let t21777 = t7414 * t352;
    let t21779 = t11 * t557 * t21777;
    let t21782 = t1953 * t557 * t21219;
    (t21771, t21775, t21776, t21777, t21779, t21782)
}
