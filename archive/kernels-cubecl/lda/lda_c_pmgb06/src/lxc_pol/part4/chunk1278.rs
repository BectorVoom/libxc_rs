//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1278/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1278<F: Float>(t13026: F, t13031: F, t15872: F, t13020: F, t15880: F, t5084: F, t2381: F, t332: F, t477: F, t5083: F, t5077: F, t5094: F) -> (F, F, F, F) {
    let t16809 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t13026 * t13031 * t15872;
    let t16812 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13020 * t5084 * t15880;
    let t16814 = t2381 * t477 * t332;
    let t16817 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5083 * t5084 * t16814;
    let t16820 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5077 * t5094 * t16814;
    (t16809, t16812, t16817, t16820)
}
