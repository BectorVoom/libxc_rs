//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1180/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1180<F: Float>(t146: F, t4918: F, t9712: F, t1575: F, t2918: F, t10006: F, t12547: F, t13382: F, t13386: F, t13390: F, t13394: F, t13399: F, t13402: F, t13405: F, t13565: F, t9503: F, t9505: F, t9577: F, t9960: F, t9962: F, t9974: F, t9981: F, t9986: F, t9987: F) -> F {
    let t14150 = t146 * t9712 * t4918;
    let t14152 = t1575 * t2918;
    let t14160 = F::cast_from(0.0044444444444444444_f64) * t9960 + F::cast_from(0.0019753086419753087_f64) * t9962 - F::cast_from(0.008888888888888889_f64) * t9974 - F::cast_from(0.5038833333333333_f64) * t13382 + t9981 - F::cast_from(0.11997222222222222_f64) * t13386 + F::cast_from(0.4319_f64) * t13390 - F::cast_from(0.64785_f64) * t13394 + F::cast_from(0.09597777777777777_f64) * t9577 + F::cast_from(0.023994444444444443_f64) * t9503 - F::cast_from(0.07198333333333333_f64) * t9505 + t9986 - F::cast_from(0.02666666666666667_f64) * t9987 - F::cast_from(0.10666666666666667_f64) * t14150 + F::cast_from(0.04_f64) * t13565 * t14152 * t12547 + F::cast_from(0.0044444444444444444_f64) * t10006 + F::cast_from(1.1757277777777777_f64) * t13399 + F::cast_from(0.14396666666666666_f64) * t13402 - F::cast_from(0.4319_f64) * t13405;
    t14160
}
