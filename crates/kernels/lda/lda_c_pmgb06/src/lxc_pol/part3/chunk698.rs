//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 698/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk698<F: Float>(t5: F, t247: F, t902: F, t2142: F, t686: F, t248: F, t2158: F, t643: F, t3912: F, t760: F, t1: F, t1068: F, t1069: F, t1074: F, t2125: F, t2128: F, t395: F, t4367: F, t9: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t4472 = t247 * t902;
    let t4481 = t2142 * t686;
    let t4483 = F::cast_from(2.0_f64) * t248 * t4481;
    let t4485 = F::cast_from(8.0_f64) * t643 * t2158;
    let t4486 = t3912 * t760;
    let t4489 = t1068 * t1;
    let t4499 = piecewise3::<F>(t6, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4486 * t1069 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4489 * t4367 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2125 * t1074 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t9 * t395 - F::cast_from(8.0_f64) * t2128 * t247);
    (t4472, t4481, t4483, t4485, t4486, t4489, t4499)
}
