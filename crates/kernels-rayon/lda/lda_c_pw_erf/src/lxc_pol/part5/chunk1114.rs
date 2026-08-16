//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1114/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1114(t12031: f64, t20737: f64, t4488: f64, t17637: f64, t2030: f64, t3965: f64, t1972: f64, t2328: f64, t4722: f64, t1967: f64, t5146: f64, t12136: f64, t6771: f64) -> (f64, f64, f64, f64, f64) {
    let t20740 = 32.0_f64 / 27.0_f64 * t4488 * t12031 * t20737;
    let t20743 = 8.0_f64 / 15.0_f64 * t3965 * t17637 * t2030;
    let t20747 = 16.0_f64 / 15.0_f64 * t3965 * t4722 * t2328 * t1972;
    let t20751 = 8.0_f64 / 9.0_f64 * t3965 * t5146 * t2328 * t1967;
    let t20753 = 16.0_f64 / 15.0_f64 * t12136 * t6771;
    (t20740, t20743, t20747, t20751, t20753)
}
