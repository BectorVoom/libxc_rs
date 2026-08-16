//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 840/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk840<F: Float>(t1289: F, t384: F, t3620: F, t73: F, t1530: F, t9: F, t1: F, t642: F, t14: F, t1413: F, t2789: F, t297: F, t301: F, t398: F) -> (F, F, F, F, F, F) {
    let t8110 = t384 * t1289;
    let t8115 = t73 * t3620;
    let t8119 = F::cast_from(1.0_f64) / t9 / t1530;
    let t8131 = t1 * t642;
    let t8139 = F::cast_from(1.0_f64) / t14 / t1413;
    let t8163 = t297 * t398 * t2789 * t301;
    (t8110, t8115, t8119, t8131, t8139, t8163)
}
