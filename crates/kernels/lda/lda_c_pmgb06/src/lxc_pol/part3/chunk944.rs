//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 944/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk944<F: Float>(t12: F, t1080: F, t395: F, t247: F, t337: F, t1083: F, t2136: F, t642: F, t1: F, t1079: F, t14: F, t2133: F, t2912: F, t2938: F, t3139: F, t3922: F, t4500: F, t4503: F, t764: F, t8499: F, zeta_threshold: F) -> (F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t11039 = t395 * t1080;
    let t11047 = t247 * t337;
    let t11050 = t395 * t1083;
    let t11058 = F::cast_from(32.0_f64) * t2136 * t642;
    let t11060 = piecewise3::<F>(t13, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t8499 * t764 * t2912 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3922 * t1 * t11039 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4500 * t3139 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1079 * t395 * t337 + F::cast_from(8.0_f64) * t4503 * t11047 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4503 * t11050 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2133 * t2938 + F::cast_from(16.0_f64) * t14 * t247 - t11058);
    (t11039, t11047, t11050, t11060)
}
