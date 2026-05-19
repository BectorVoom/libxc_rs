//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1449/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1449<F: Float>(t12: F, t1080: F, t1083: F, t1100: F, t11282: F, t1219: F, t2200: F, t2386: F, t2389: F, t2799: F, t337: F, t3548: F, t395: F, t4378: F, t5423: F, t5966: F, t5971: F, t5974: F, t79: F, t8139: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t18566 = piecewise3::<F>(t13, F::new(0.0), -F::new(56.0) / F::new(81.0) * t8139 * t2386 * t1080 - F::new(64.0) / F::new(27.0) * t4378 * t5423 + F::new(8.0) / F::new(27.0) * t5966 * t1083 - F::new(16.0) / F::new(9.0) * t1219 * t79 * t1100 + F::new(8.0) / F::new(9.0) * t2200 * t395 - F::new(8.0) / F::new(3.0) * t2200 * t2799 + F::new(8.0) / F::new(27.0) * t3548 * t2389 * t1080 - F::new(4.0) / F::new(9.0) * t1219 * t5974 * t337 - F::new(2.0) / F::new(9.0) * t5971 * t1083 - t11282);
    t18566
}
