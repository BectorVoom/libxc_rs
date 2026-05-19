//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 695/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk695<F: Float>(t5: F, t12: F, t1072: F, t1941: F, t332: F, t594: F, t5961: F, t6329: F, t6334: F, t2386: F, t336: F, t15: F, t2389: F, t1949: F, t337: F, t5974: F, t598: F, zeta_threshold: F) -> (F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t6340 = piecewise3::<F>(t6, F::new(0.0), F::new(80.0) / F::new(27.0) * t6329 * t332 + F::new(160.0) / F::new(9.0) * t1941 * t1072 + F::new(40.0) / F::new(9.0) * t6334 * t332 + F::new(8.0) / F::new(3.0) * t594 * t5961);
    let t6341 = t336 * t2386;
    let t6346 = t15 * t2389;
    let t6352 = piecewise3::<F>(t13, F::new(0.0), F::new(80.0) / F::new(27.0) * t6341 * t337 - F::new(160.0) / F::new(9.0) * t1949 * t1072 + F::new(40.0) / F::new(9.0) * t6346 * t337 + F::new(8.0) / F::new(3.0) * t598 * t5974);
    (t6340, t6341, t6352)
}
