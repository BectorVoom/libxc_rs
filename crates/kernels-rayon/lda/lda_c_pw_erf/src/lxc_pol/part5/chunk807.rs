//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 807/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk807(t6065: f64, t6067: f64, t6072: f64, t7376: f64, t87: f64, t40: f64, t3139: f64, t3151: f64, t3157: f64, t3159: f64, t3162: f64, t3168: f64, t3170: f64, t3174: f64, t3176: f64) -> (f64, f64, f64) {
    let t7380 = 12.0_f64 * t6065;
    let t7381 = 12.0_f64 * t6067;
    let t7382 = 3.0_f64 * t6072;
    let t7383 = t7376 * t87;
    let t7384 = t40 * t7383;
    let t7385 = -t3139 + t3151 - t7380 - t3157 - t3159 - t7381 - t3162 - t3168 + t7382 + t7384 + t3170 + t3174 + t3176;
    (t7383, t7384, t7385)
}
