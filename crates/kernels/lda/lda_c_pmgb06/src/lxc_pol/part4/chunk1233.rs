//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1233/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1233<F: Float>(t16241: F, t12232: F, t12234: F, t161: F, t166: F, t2093: F, t4935: F, t1447: F, t6114: F, t1972: F, t5319: F, t14347: F, t14350: F, t14353: F, t14356: F, t14359: F, t16228: F, t16237: F, t16239: F) -> (F, F, F, F, F, F, F) {
    let t16242 = F::new(2.0) / F::new(45.0) * t16241;
    let t16243 = F::new(8.0) / F::new(405.0) * t12232;
    let t16244 = F::new(8.0) / F::new(405.0) * t12234;
    let t16248 = t161 * t166 * t2093 * t4935 / F::new(15.0);
    let t16249 = t1447 * t6114;
    let t16250 = F::new(4.0) / F::new(45.0) * t16249;
    let t16252 = F::new(2.0) / F::new(15.0) * t1972 * t5319;
    let t16253 = t16228 + F::cast_from(0.04472697096444135_f64) * t14347 + F::cast_from(0.06709045644666203_f64) * t14350 + F::cast_from(0.21642082724729686_f64) * t14353 + F::cast_from(0.8656833089891874_f64) * t14356 + F::cast_from(0.6492624817418906_f64) * t14359 + t16237 + t16239 + t16242 + t16243 + t16244 - t16248 + t16250 + t16252;
    (t16242, t16243, t16244, t16248, t16250, t16252, t16253)
}
