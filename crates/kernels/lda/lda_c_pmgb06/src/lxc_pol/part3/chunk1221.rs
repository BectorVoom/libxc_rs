//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1221/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1221<F: Float>(t187: F, t5632: F, t3389: F, t856: F, t5635: F, t5638: F, t13985: F, t13988: F, t13992: F, t13995: F, t13997: F, t13999: F, t14002: F) -> F {
    let t14465 = t5632 * t187;
    let t14467 = t856 * t3389;
    let t14469 = t5635 * t187;
    let t14471 = t5638 * t187;
    let t14472 = F::new(8.0) * t14471;
    let t14473 = t13985 - t13988 - t13992 + F::new(4.0) * t14465 + F::new(0.0011033703703703704) * t14467 + F::new(4.0) * t14469 + t14472 - t13995 - t13997 - t13999 - t14002;
    t14473
}
