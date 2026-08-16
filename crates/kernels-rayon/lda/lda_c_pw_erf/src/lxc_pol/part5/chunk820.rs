//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 820/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk820(t3439: f64, t4540: f64, t7499: f64, t7500: f64, t7501: f64, t7502: f64, t7503: f64, t7504: f64, t7505: f64, t7507: f64, t7509: f64, t7511: f64, t7512: f64, t7517: f64, t7518: f64, t7519: f64, t7524: f64) -> f64 {
    let t7526 = t7499 - t7500 - t7501 + t7502 + t7503 - t7504 - t7505 - t7507 + t7509 + t7511 + t3439 + t7512 - t7517 + t7518 + t7519 - t7524 + 0.6492624817418906_f64 * t4540;
    t7526
}
