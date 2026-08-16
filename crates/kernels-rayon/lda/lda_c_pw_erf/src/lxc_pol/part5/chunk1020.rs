//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1020/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1020(t1518: f64, t2479: f64, t548: f64, t568: f64, t6671: f64, t184: f64, t509: f64, t784: f64, t1982: f64, t2134: f64, t2561: f64, t3762: f64, t571: f64) -> (f64, f64, f64, f64, f64) {
    let t16961 = t548 * t1518 * t2479;
    let t16963 = t6671 * t568;
    let t16971 = t784 * t509 * t184;
    let t16989 = t1982 * t2134;
    let t17040 = t571 * t3762 * t2561;
    (t16961, t16963, t16971, t16989, t17040)
}
