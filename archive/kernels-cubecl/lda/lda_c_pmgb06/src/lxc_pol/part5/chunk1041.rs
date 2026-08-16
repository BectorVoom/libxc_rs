//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1041/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1041<F: Float>(t1915: F, t19340: F, t493: F, t19344: F, t1981: F, t1444: F, t7517: F, t5463: F, t7516: F, t1919: F, t19358: F, t332: F, t7284: F, t9190: F) -> (F, F, F, F, F, F) {
    let t19458 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t1915 * t19340;
    let t19461 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1981 * t1915 * t19344;
    let t19463 = t1444 * t7517 / F::cast_from(9.0_f64);
    let t19466 = t493 * t5463 * t7516 / F::cast_from(9.0_f64);
    let t19469 = t493 * t1919 * t19358 / F::cast_from(9.0_f64);
    let t19471 = t9190 * t7284 * t332;
    (t19458, t19461, t19463, t19466, t19469, t19471)
}
