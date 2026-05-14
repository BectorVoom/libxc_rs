//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1057/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1057<F: Float>(t13456: F, t13457: F, t13461: F, t13463: F, t13465: F, t13467: F, t13470: F, t13477: F, t13480: F, t13482: F, t13486: F, t13489: F, t13492: F, t13496: F, t13498: F, t13501: F, t13503: F, t13505: F, t13508: F, t13510: F, t13512: F, t13514: F, t13516: F) -> (F, F) {
    let t14430 = t13456 + t13457 + t13461 + t13463 + t13465 + t13467 - t13470 + t13477 + t13480 + t13482 + t13486;
    let t14431 = t13489 + t13492 + t13496 + t13498 + t13501 + t13503 + t13505 + t13508 + t13510 + t13512 + t13514 + t13516;
    (t14430, t14431)
}
