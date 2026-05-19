//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1201/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1201<F: Float>(t12: F, t1072: F, t1079: F, t14: F, t19395: F, t2133: F, t21345: F, t337: F, t4500: F, t4503: F, t5974: F, t6054: F, t6681: F, t7295: F, t7300: F, t8499: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t21768 = piecewise3::<F>(t13, F::new(0.0), F::new(40.0) / F::new(81.0) * t8499 * t7295 * t337 + F::new(16.0) / F::new(9.0) * t6054 * t1072 - F::new(8.0) / F::new(9.0) * t4500 * t6681 - F::new(8.0) / F::new(3.0) * t4503 * t21345 + F::new(4.0) / F::new(3.0) * t2133 * t5974 + F::new(4.0) / F::new(9.0) * t1079 * t7300 * t337 + F::new(4.0) / F::new(3.0) * t14 * t19395);
    t21768
}
