//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1077/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1077(t142: f64, t7913: f64, t455: f64, t159: f64, t285: f64, t462: f64, t7337: f64, t11635: f64, t8777: f64) -> (f64, f64, f64, f64) {
    let t20143 = t142 * t7913;
    let t20144 = t455 * t20143;
    let t20174 = t462 * t7337 * t159 * t285;
    let t20179 = -t11635 - t8777;
    (t20143, t20144, t20174, t20179)
}
