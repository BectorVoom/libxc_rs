//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 316/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk316(t1125: f64, t153: f64, t462: f64, t925: f64, t933: f64) -> f64 {
    let t1128 = 0.0023_f64 * t925 + 0.022758333333333332_f64 * t933 - 0.006097225869850511_f64 * t462 + 0.0010844166666666667_f64 * t153 * t1125;
    t1128
}
