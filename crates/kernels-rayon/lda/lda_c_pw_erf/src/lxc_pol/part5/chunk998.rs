//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 998/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk998(t1446: f64, t6696: f64, t1449: f64, t519: f64, t6938: f64, t4804: f64, t6689: f64, t3794: f64, t518: f64, t6787: f64, t2146: f64, t5342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15538 = t1446 * t6696;
    let t15542 = t519 * t1449 * t6938;
    let t15557 = t4804 * t6689;
    let t15559 = t3794 * t6689;
    let t15563 = t6787 * t518;
    let t15568 = t2146 * t5342;
    (t15538, t15542, t15557, t15559, t15563, t15568)
}
