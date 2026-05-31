//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1111/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1111<F: Float>(t17964: F, t1992: F, t493: F, t851: F, t1972: F, t6287: F, t2088: F, t6112: F, t1444: F, t7685: F, t5179: F, t7684: F) -> (F, F, F, F, F) {
    let t20353 = t493 * t1992 * t17964 * t851 / F::cast_from(5.0_f64);
    let t20355 = F::cast_from(3.0_f64) / F::cast_from(5.0_f64) * t1972 * t6287;
    let t20359 = t493 * t1992 * t6112 * t2088 / F::cast_from(5.0_f64);
    let t20361 = t1444 * t7685 / F::cast_from(5.0_f64);
    let t20364 = t493 * t5179 * t7684 / F::cast_from(5.0_f64);
    (t20353, t20355, t20359, t20361, t20364)
}
