//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1180/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1180(t13884: f64, t13886: f64, t13888: f64, t13890: f64, t13892: f64, t13897: f64, t13899: f64, t13901: f64, t13903: f64, t13905: f64, t13907: f64, t13909: f64, t13911: f64) -> f64 {
    let t13912 = -t13884 - t13886 - t13888 - t13890 - t13892 - t13897 - t13899 + t13901 + t13903 + t13905 - t13907 - t13909 + t13911;
    t13912
}
