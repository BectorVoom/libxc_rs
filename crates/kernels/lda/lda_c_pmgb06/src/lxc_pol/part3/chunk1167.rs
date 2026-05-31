//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1167/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1167<F: Float>(t13933: F, t2961: F, t439: F, t1972: F, t2877: F, t2876: F, t493: F, t5486: F, t1444: F, t5359: F, t1380: F, t2912: F, t5280: F) -> (F, F, F, F, F) {
    let t13936 = t439 * t13933 * t2961 / F::cast_from(9.0_f64);
    let t13938 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t2877;
    let t13941 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t5486 * t2876;
    let t13943 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1444 * t5359;
    let t13947 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t1380 * t5280 * t2912;
    (t13936, t13938, t13941, t13943, t13947)
}
