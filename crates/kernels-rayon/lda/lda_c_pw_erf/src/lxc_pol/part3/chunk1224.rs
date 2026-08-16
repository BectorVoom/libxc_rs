//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1224/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1224(t11383: f64, t11388: f64, t11390: f64, t11392: f64, t11402: f64, t11404: f64, t11406: f64, t11462: f64, t11464: f64, t8373: f64, t8382: f64, t8386: f64, t8389: f64, t8393: f64, t8397: f64, t8400: f64) -> f64 {
    let t14421 = -t8373 - t11383 - t8382 + t8386 - t11388 + t11390 - t11392 - t8389 - t8393 + t8397 - t8400 + t11402 + t11404 - t11406 + t11462 + t11464;
    t14421
}
