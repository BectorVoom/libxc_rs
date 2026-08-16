//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 934/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk934<F: Float>(t1795: F, t868: F, t2422: F, t391: F, t2414: F, t81: F, t199: F, t1799: F, t1808: F, t2454: F, t399: F, t4187: F, t4212: F, t4214: F, t5542: F, t5551: F, t5553: F, t566: F, t6928: F, t6939: F, t795: F, t84: F) -> (F, F) {
    let t6942 = t1795 * t868;
    let t6944 = t391 * t2422;
    let t6946 = t81 * t2414;
    let t6947 = t6946 * t199;
    let t6951 = t5542 + t5551 + t5553 - F::cast_from(0.0837628205355044_f64) * t6928 * t199 - F::cast_from(0.0837628205355044_f64) * t2454 * t566 - F::cast_from(0.1675256410710088_f64) * t1799 * t868 - F::cast_from(0.1675256410710088_f64) * t795 * t1808 - F::cast_from(0.0837628205355044_f64) * t399 * t2422 - F::cast_from(0.0837628205355044_f64) * t84 * t6939 + F::cast_from(0.1675256410710088_f64) * t6942 + F::cast_from(0.0837628205355044_f64) * t6944 + F::cast_from(0.0837628205355044_f64) * t6947 + t4187 - F::cast_from(0.1675256410710088_f64) * t4212 - F::cast_from(0.1675256410710088_f64) * t4214;
    (t6946, t6951)
}
