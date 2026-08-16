//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 742/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk742(t2002: f64, t4753: f64, t1310: f64, t2146: f64, t2151: f64, t219: f64) -> (f64, f64, f64) {
    let t4755 = 16.0_f64 / 45.0_f64 * t4753 * t2002;
    let t4757 = 8.0_f64 / 45.0_f64 * t2146 * t1310;
    let t4758 = t2151 * t219;
    (t4755, t4757, t4758)
}
