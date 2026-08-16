//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 965/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk965(t248: f64, t256: f64, t4606: f64, t5021: f64, t3704: f64, t665: f64, t1124: f64, t265: f64, t266: f64, t3990: f64, t640: f64, t653: f64) -> (f64, f64, f64, f64, f64) {
    let t11088 = t248 * (-0.33530864197530863_f64 * t4606 + 1.8360493827160493_f64 * t5021) * t256 / 3.0_f64;
    let t11093 = t665 * t3704;
    let t11097 = 56.0_f64 / 1215.0_f64 * t265 * t266 * t1124;
    let t11098 = t640 * t3990;
    let t11101 = 32.0_f64 / 81.0_f64 * t653 * t3990;
    (t11088, t11093, t11097, t11098, t11101)
}
