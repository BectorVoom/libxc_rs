//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 797/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk797(t6125: f64, t7060: f64, t7079: f64, t7305: f64, t312: f64, t19: f64, t2686: f64, t729: f64, t734: f64, t5968: f64, t4387: f64, t4389: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7307 = t6125 + t7060 + t7079 + t7305;
    let t7308 = t7307 * t312;
    let t7314 = t2686 * t729 * t19;
    let t7315 = t7314 * t734;
    let t7323 = 1.7544670192365612_f64 * t5968;
    let t7324 = 0.0007324622014701264_f64 * t4387;
    let t7325 = 1.7544670192365612_f64 * t4389;
    (t7307, t7308, t7314, t7315, t7323, t7324, t7325)
}
