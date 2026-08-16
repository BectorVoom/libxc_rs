//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1125/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1125(t4488: f64, t4490: f64, t6460: f64, t16863: f64, t2026: f64, t3965: f64, t13115: f64, t14034: f64, t2388: f64, t4475: f64, t6400: f64, t15697: f64) -> (f64, f64, f64, f64, f64) {
    let t20876 = 16.0_f64 / 15.0_f64 * t4488 * t4490 * t6460;
    let t20879 = 16.0_f64 / 15.0_f64 * t3965 * t16863 * t2026;
    let t20882 = 16.0_f64 / 15.0_f64 * t13115 * t14034 * t2388;
    let t20885 = 32.0_f64 / 15.0_f64 * t13115 * t4475 * t6400;
    let t20886 = 16.0_f64 / 45.0_f64 * t15697;
    (t20876, t20879, t20882, t20885, t20886)
}
