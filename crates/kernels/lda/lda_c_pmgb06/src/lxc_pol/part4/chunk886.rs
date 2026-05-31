//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 886/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk886<F: Float>(t12: F, t2386: F, t336: F, t15: F, t2389: F, t1072: F, t1949: F, t337: F, t5974: F, t598: F, t44: F, t6340: F, t2519: F, t607: F, zeta_threshold: F) -> (F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t6341 = t336 * t2386;
    let t6346 = t15 * t2389;
    let t6352 = piecewise3::<F>(t13, F::cast_from(0.0_f64), F::cast_from(80.0_f64) / F::cast_from(27.0_f64) * t6341 * t337 - F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t1949 * t1072 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t6346 * t337 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t598 * t5974);
    let t6355 = (t6340 / F::cast_from(2.0_f64) + t6352 / F::cast_from(2.0_f64)) * t44;
    let t6358 = t2519 * t607;
    (t6341, t6346, t6355, t6358)
}
