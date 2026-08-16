//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 119/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk119<F: Float>(t280: F, t347: F, t287: F, t294: F, t305: F) -> (F, F, F, F) {
    let t348 = t347 * t280;
    let t353 = F::cast_from(1.6042420957638404_f64) * t287 + F::cast_from(0.64_f64) * t294 + F::cast_from(0.07519884823893001_f64);
    let t354 = F::ln(t353);
    let t355 = t354 * t305;
    (t348, t353, t354, t355)
}
