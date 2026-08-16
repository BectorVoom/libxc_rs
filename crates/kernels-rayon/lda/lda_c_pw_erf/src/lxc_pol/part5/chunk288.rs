//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 288/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk288(t371: f64, t973: f64, t920: f64, t923: f64, t925: f64, t929: f64, t931: f64, t933: f64) -> (f64, f64) {
    let t974 = t973 * t371;
    let t983 = -0.7843833333333333_f64 * t920 + 1.5687666666666666_f64 * t923 + 0.6886333333333333_f64 * t925 + 0.14025833333333335_f64 * t929 + 0.2805166666666667_f64 * t931 + 0.17365833333333333_f64 * t933;
    (t974, t983)
}
