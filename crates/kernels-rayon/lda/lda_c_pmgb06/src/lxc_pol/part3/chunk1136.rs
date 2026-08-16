//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1136/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1136(t13502: f64, t1447: f64, t4585: f64, t13477: f64, t13480: f64, t13482: f64, t13486: f64, t13489: f64, t13492: f64, t13496: f64, t13498: f64, t13501: f64) -> (f64, f64, f64) {
    let t13503 = 4.0_f64 / 45.0_f64 * t13502;
    let t13504 = t1447 * t4585;
    let t13505 = 2.0_f64 / 45.0_f64 * t13504;
    let t13506 = t13477 + t13480 + t13482 + t13486 + t13489 + t13492 + t13496 + t13498 + t13501 + t13503 + t13505;
    (t13503, t13505, t13506)
}
