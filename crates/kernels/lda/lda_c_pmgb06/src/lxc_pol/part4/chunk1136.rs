//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1136/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1136<F: Float>(t12: F, t1079: F, t1080: F, t1083: F, t1100: F, t11058: F, t2133: F, t2386: F, t2389: F, t2799: F, t337: F, t3922: F, t395: F, t4500: F, t5423: F, t5974: F, t6054: F, t6059: F, t79: F, t8499: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t14933 = piecewise3::<F>(t13, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t8499 * t2386 * t1080 + F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t4500 * t5423 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t6054 * t1083 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t1079 * t79 * t1100 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t2133 * t395 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t2133 * t2799 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t3922 * t2389 * t1080 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1079 * t5974 * t337 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t6059 * t1083 - t11058);
    t14933
}
