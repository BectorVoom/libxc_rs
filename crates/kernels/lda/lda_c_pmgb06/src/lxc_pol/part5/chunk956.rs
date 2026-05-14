//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 956/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk956<F: Float>(t1447: F, t7521: F, t1919: F, t19362: F, t1981: F, t1444: F, t10293: F, t493: F, t7520: F, t2088: F, t2541: F, t2991: F, t529: F, t7598: F, t19371: F, t5470: F) -> (F, F, F, F, F, F, F) {
    let t20008 = t1447 * t7521;
    let t20009 = 2.0 / 27.0 * t20008;
    let t20012 = 2.0 / 9.0 * t1981 * t1919 * t19362;
    let t20014 = t1444 * t7521 / 9.0;
    let t20017 = t493 * t10293 * t7520 / 9.0;
    let t20021 = t493 * t2991 * t2541 * t2088 / 9.0;
    let t20025 = 2.0 / 9.0 * t493 * t2991 * t7598 * t529;
    let t20028 = 32.0 / 27.0 * t493 * t5470 * t19371;
    (t20009, t20012, t20014, t20017, t20021, t20025, t20028)
}
