//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1243/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1243<F: Float>(t12456: F, t12460: F, t12462: F, t12465: F, t1462: F, t1465: F, t1981: F, t764: F, t1963: F, t5220: F, t2591: F, t607: F) -> (F, F, F, F, F, F, F) {
    let t16372 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t12456;
    let t16373 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t12460;
    let t16374 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t12462;
    let t16375 = F::cast_from(128.0_f64) / F::cast_from(405.0_f64) * t12465;
    let t16379 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1981 * t1462 * t1465 * t764;
    let t16380 = t5220 * t1963;
    let t16381 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t16380;
    let t16382 = t2591 * t607;
    (t16372, t16373, t16374, t16375, t16379, t16381, t16382)
}
