//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1211/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1211<F: Float>(t12: F, t1072: F, t1219: F, t15: F, t19395: F, t1949: F, t21345: F, t337: F, t4700: F, t5974: F, t598: F, t6341: F, t6681: F, t7295: F, t7300: F, t765: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t21891 = piecewise3::<f64>(t13, F::new(0.0), -F::new(80.0) / F::new(81.0) * t1219 * t7295 * t337 - F::new(160.0) / F::new(9.0) * t6341 * t1072 + F::new(80.0) / F::new(9.0) * t765 * t6681 - F::new(80.0) / F::new(3.0) * t4700 * t21345 + F::new(40.0) / F::new(3.0) * t1949 * t5974 + F::new(40.0) / F::new(9.0) * t15 * t7300 * t337 + F::new(8.0) / F::new(3.0) * t598 * t19395);
    t21891
}
