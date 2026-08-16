//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1214/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1214(t13456: f64, t13457: f64, t13461: f64, t13463: f64, t13465: f64, t13467: f64, t13470: f64, t13477: f64, t13480: f64, t13482: f64, t13486: f64, t13489: f64, t13492: f64, t13496: f64, t13498: f64, t13501: f64, t13503: f64, t13505: f64, t13508: f64, t13510: f64, t13512: f64, t13514: f64, t13516: f64) -> (f64, f64) {
    let t14430 = t13456 + t13457 + t13461 + t13463 + t13465 + t13467 - t13470 + t13477 + t13480 + t13482 + t13486;
    let t14431 = t13489 + t13492 + t13496 + t13498 + t13501 + t13503 + t13505 + t13508 + t13510 + t13512 + t13514 + t13516;
    (t14430, t14431)
}
