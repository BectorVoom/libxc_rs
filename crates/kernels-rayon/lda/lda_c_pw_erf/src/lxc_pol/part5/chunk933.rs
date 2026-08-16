//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 933/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk933(t1423: f64, t3927: f64, t24: f64, t247: f64, t3932: f64, t645: f64, t256: f64, t639: f64, t248: f64, t4606: f64, t5021: f64, t3704: f64, t665: f64) -> (f64, f64, f64, f64, f64) {
    let t11074 = t1423 * t3927;
    let t11079 = 0.2431111111111111_f64 * t645 * t24 * t247 * t3932;
    let t11081 = t639 * t3932 * t256;
    let t11088 = t248 * (-0.33530864197530863_f64 * t4606 + 1.8360493827160493_f64 * t5021) * t256 / 3.0_f64;
    let t11093 = t665 * t3704;
    (t11074, t11079, t11081, t11088, t11093)
}
