//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1184/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1184(t1381: f64, t3974: f64, t3976: f64, t5155: f64, t13115: f64, t13116: f64, t593: f64, t10027: f64, t5162: f64, t12475: f64, t12492: f64, t5147: f64) -> (f64, f64, f64, f64) {
    let t13952 = 8.0_f64 / 15.0_f64 * t3974 * t3976 * t5155 * t1381;
    let t13956 = 32.0_f64 / 15.0_f64 * t13115 * t3976 * t13116 * t593;
    let t13958 = 32.0_f64 / 15.0_f64 * t10027 * t5162;
    let t13961 = 32.0_f64 / 9.0_f64 * t12475 * t5147 * t12492;
    (t13952, t13956, t13958, t13961)
}
