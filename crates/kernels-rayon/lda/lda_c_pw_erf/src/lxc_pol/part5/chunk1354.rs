//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1354/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1354(t14096: f64, t14099: f64, t14100: f64, t14103: f64, t14105: f64, t22954: f64, t22958: f64, t22961: f64, t22967: f64, t22971: f64, t22975: f64, t22978: f64, t22981: f64) -> f64 {
    let t23330 = t22954 - t22958 - t22961 + t22967 + t22971 - t22975 + t22978 + t22981 + t14096 + t14099 + 0.9738937226128359_f64 * t14100 + 0.10063568466999305_f64 * t14103 - 0.03354522822333102_f64 * t14105;
    t23330
}
