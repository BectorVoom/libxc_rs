//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1293/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1293(t13152: f64, t13154: f64, t13162: f64, t13164: f64, t13166: f64, t13171: f64, t13175: f64, t13177: f64, t13179: f64, t13182: f64, t13187: f64, t13189: f64, t13191: f64) -> f64 {
    let t15072 = t13152 + t13154 - t13162 + t13164 - t13166 + t13171 + t13175 - t13177 + t13179 + t13182 - t13187 - t13189 + t13191;
    t15072
}
