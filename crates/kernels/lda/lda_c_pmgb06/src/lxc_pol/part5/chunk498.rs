//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 498/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk498<F: Float>(t5: F, t12: F, t10: F, t2377: F, t2381: F, t594: F, t15: F, t2386: F, t2389: F, t598: F, t44: F, t1929: F, t1931: F, t1934: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t2510 = piecewise3::<f64>(t6, F::new(0.0), F::new(40.0) / F::new(9.0) * t10 * t2377 + F::new(8.0) / F::new(3.0) * t594 * t2381);
    let t2516 = piecewise3::<f64>(t13, F::new(0.0), F::new(40.0) / F::new(9.0) * t15 * t2386 + F::new(8.0) / F::new(3.0) * t598 * t2389);
    let t2519 = (t2510 / F::new(2.0) + t2516 / F::new(2.0)) * t44;
    let t2522 = F::new(2.0) / F::new(45.0) * t1929;
    let t2523 = F::new(2.0) / F::new(45.0) * t1931;
    let t2524 = F::new(2.0) / F::new(45.0) * t1934;
    (t2519, t2522, t2523, t2524)
}
