//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1262/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1262(t11020: f64, t11022: f64, t11025: f64, t11027: f64, t11029: f64, t12066: f64, t12070: f64, t12075: f64, t12078: f64, t12082: f64, t12084: f64, t12085: f64, t12086: f64) -> f64 {
    let t14991 = t12066 + t12070 - t12075 - t12078 + t12082 + 0.299209_f64 * t11020 - 0.19947266666666666_f64 * t11022 - t11025 + t11027 + t11029 + t12084 + t12085 - t12086;
    t14991
}
