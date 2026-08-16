//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1343/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1343(t22288: f64, t22292: f64, t22296: f64, t22299: f64, t22302: f64, t22304: f64, t22307: f64, t22310: f64, t22313: f64, t22317: f64, t22322: f64, t22327: f64, t22330: f64) -> f64 {
    let t23302 = t22288 + t22292 + t22296 + t22299 + t22302 + t22304 + t22307 + t22310 + t22313 - t22317 - t22322 + t22327 - t22330;
    t23302
}
