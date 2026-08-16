//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 921/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk921(t1634: f64, t1638: f64, t635: f64, t1125: f64, t602: f64, t603: f64, t4192: f64, t4207: f64, t163: f64, t169: f64, t234: f64, t2817: f64) -> (f64, f64, f64, f64) {
    let t10715 = 0.04472697096444135_f64 * t1638 * t635 * t1634;
    let t10718 = 0.2244364134416412_f64 * t602 * t1125 * t603;
    let t10719 = t4192 * t4207;
    let t10749 = 0.4097848972398244_f64 * t169 * t2817 * t234 * t163;
    (t10715, t10718, t10719, t10749)
}
