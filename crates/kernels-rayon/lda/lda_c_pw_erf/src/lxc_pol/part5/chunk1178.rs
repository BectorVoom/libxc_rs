//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1178/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1178(t2325: f64, t784: f64, t3965: f64, t4501: f64, t542: f64, t12031: f64, t348: f64, t12475: f64, t34: f64, t5147: f64, t739: f64, t21398: f64) -> (f64, f64, f64, f64) {
    let t21451 = t2325 * t784;
    let t21455 = 8.0_f64 / 9.0_f64 * t3965 * t4501 * t21451 * t542;
    let t21459 = 64.0_f64 / 27.0_f64 * t3965 * t12031 * t21451 * t348;
    let t21464 = 32.0_f64 / 9.0_f64 * t12475 * t5147 * t739 * t784 * t34;
    let t21467 = 8.0_f64 / 9.0_f64 * t3965 * t4501 * t21398;
    (t21455, t21459, t21464, t21467)
}
