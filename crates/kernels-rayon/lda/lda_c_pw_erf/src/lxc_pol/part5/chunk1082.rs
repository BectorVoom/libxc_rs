//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1082/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1082(t11382: f64, t11388: f64, t11404: f64, t20070: f64, t20074: f64, t20075: f64, t20077: f64, t20078: f64, t20079: f64, t8373: f64, t8382: f64, t8386: f64, t8389: f64, t8393: f64, t8397: f64, t8400: f64) -> f64 {
    let t20195 = t20070 - t20074 - t20075 - t11382 - t8373 - t8382 + t8386 - t11388 - t8389 - t8393 + t8397 - t8400 + t20077 - t11404 - t20078 + t20079;
    t20195
}
