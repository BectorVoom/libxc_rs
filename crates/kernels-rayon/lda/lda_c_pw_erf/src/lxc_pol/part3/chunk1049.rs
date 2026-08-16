//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1049/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1049(t4688: f64, t954: f64, t4758: f64, t571: f64, t2178: f64, t3709: f64, t4804: f64, t5394: f64, t1450: f64, t5327: f64, t518: f64, t5214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12289 = t4688 * t954;
    let t12292 = 16.0_f64 / 15.0_f64 * t571 * t4758 * t12289;
    let t12294 = 8.0_f64 / 15.0_f64 * t3709 * t2178;
    let t12296 = 4.0_f64 / 5.0_f64 * t4804 * t5394;
    let t12297 = t5327 * t1450;
    let t12298 = 16.0_f64 / 45.0_f64 * t12297;
    let t12299 = t5214 * t518;
    (t12289, t12292, t12294, t12296, t12298, t12299)
}
