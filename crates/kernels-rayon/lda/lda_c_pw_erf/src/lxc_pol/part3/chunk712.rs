//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 712/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk712(t1314: f64, t4490: f64, t4488: f64, t1251: f64, t3966: f64) -> (f64, f64, f64) {
    let t4491 = t4490 * t1314;
    let t4493 = 16.0_f64 / 45.0_f64 * t4488 * t4491;
    let t4494 = t3966 * t1251;
    (t4491, t4493, t4494)
}
