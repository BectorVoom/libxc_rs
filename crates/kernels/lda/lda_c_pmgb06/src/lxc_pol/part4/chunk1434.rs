//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1434/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1434<F: Float>(t12: F, t1080: F, t1083: F, t1100: F, t12960: F, t15: F, t1949: F, t2441: F, t2443: F, t2799: F, t337: F, t395: F, t5423: F, t5974: F, t6341: F, t6346: F, t765: F, t79: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t18377 = piecewise3::<f64>(t13, F::new(0.0), -F::new(80.0) / F::new(81.0) * t2441 * t1080 - F::new(640.0) / F::new(27.0) * t765 * t5423 + F::new(80.0) / F::new(27.0) * t6341 * t1083 + F::new(320.0) / F::new(9.0) * t15 * t79 * t1100 - F::new(160.0) / F::new(9.0) * t1949 * t395 + F::new(160.0) / F::new(3.0) * t1949 * t2799 + F::new(80.0) / F::new(27.0) * t2443 * t1080 + F::new(80.0) / F::new(9.0) * t15 * t5974 * t337 + F::new(40.0) / F::new(9.0) * t6346 * t1083 - t12960);
    t18377
}
