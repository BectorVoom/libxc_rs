//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1309/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1309(t13958: f64, t13961: f64, t13965: f64, t13969: f64, t13972: f64, t13974: f64, t13976: f64, t13978: f64, t13979: f64, t13980: f64, t13981: f64, t13983: f64, t13984: f64) -> f64 {
    let t15114 = -t13958 + t13961 + t13965 - t13969 + t13972 - t13974 + t13976 + t13978 - t13979 - t13980 - t13981 + t13983 + t13984;
    t15114
}
