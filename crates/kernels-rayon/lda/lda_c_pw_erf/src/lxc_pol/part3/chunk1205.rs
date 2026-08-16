//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1205/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1205(t10567: f64, t197: f64, t11724: f64, t519: f64, t3892: f64, t473: f64, t11729: f64, t1446: f64, t5257: f64, t5261: f64, t1313: f64, t4748: f64, t945: f64) -> (f64, f64, f64, f64, f64) {
    let t14200 = t10567 * t197;
    let t14203 = 352.0_f64 / 243.0_f64 * t519 * t14200 * t11724;
    let t14205 = t473 * t3892 * t197;
    let t14208 = 64.0_f64 / 27.0_f64 * t519 * t14205 * t11729;
    let t14210 = 16.0_f64 / 9.0_f64 * t1446 * t5257;
    let t14212 = 4.0_f64 / 15.0_f64 * t1446 * t5261;
    let t14216 = 4.0_f64 / 15.0_f64 * t519 * t1313 * t4748 * t945;
    (t14203, t14208, t14210, t14212, t14216)
}
