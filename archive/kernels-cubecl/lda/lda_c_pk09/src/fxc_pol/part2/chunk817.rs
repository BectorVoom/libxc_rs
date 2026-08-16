//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 817/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk817<F: Float>(t8192: F, t8202: F, t8214: F, t8229: F, t188: F, t2192: F, t3928: F, t3933: F, t659: F, t694: F, t8169: F, t8171: F, t8176: F) -> (F, F) {
    let t8231 = t8192 + t8202 + t8214 + t8229;
    let t8234 = t8169 * t188 - t8171 * t694 / F::cast_from(2.0_f64) - t3928 * t2192 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3933 * t8176 - t659 * t8231 / F::cast_from(2.0_f64);
    (t8231, t8234)
}
