//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1215/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1215(t21927: f64, t1318: f64, t1466: f64, t16907: f64, t811: f64, t34: f64, t4892: f64, t6188: f64, t4753: f64, t7570: f64, t3416: f64, t3899: f64, t7596: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21928 = 32.0_f64 / 45.0_f64 * t21927;
    let t21932 = 4.0_f64 / 5.0_f64 * t1318 * t1466 * t16907 * t811;
    let t21936 = 4.0_f64 / 5.0_f64 * t1318 * t4892 * t6188 * t34;
    let t21938 = 4.0_f64 / 5.0_f64 * t4753 * t7570;
    let t21940 = 4.0_f64 / 5.0_f64 * t3416 * t7570;
    let t21942 = t1318 * t3899 * t7596;
    (t21928, t21932, t21936, t21938, t21940, t21942)
}
