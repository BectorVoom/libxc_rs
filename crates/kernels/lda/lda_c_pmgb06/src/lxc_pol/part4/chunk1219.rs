//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1219/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1219<F: Float>(t1444: F, t6403: F, t493: F, t5447: F, t6402: F, t1083: F, t2541: F, t1915: F, t9402: F, t16040: F, t16044: F, t16048: F, t16050: F, t16052: F, t16054: F, t16056: F, t16058: F, t16060: F, t16063: F, t16067: F) -> (F, F, F, F, F, F) {
    let t16069 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1444 * t6403;
    let t16072 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t493 * t5447 * t6402;
    let t16073 = t2541 * t1083;
    let t16076 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t1915 * t16073;
    let t16077 = t9402 / F::cast_from(135.0_f64);
    let t16078 = -t16040 - t16044 - t16048 - t16050 + t16052 + t16054 - t16056 - t16058 + t16060 + t16063 + t16067 + t16069 + t16072 + t16076 - t16077;
    (t16069, t16072, t16073, t16076, t16077, t16078)
}
