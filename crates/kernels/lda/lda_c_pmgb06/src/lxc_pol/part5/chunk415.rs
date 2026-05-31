//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 415/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk415<F: Float>(t1972: F, t500: F, t1444: F, t835: F, t1450: F, t834: F) -> (F, F, F) {
    let t1974 = t1972 * t500 / F::cast_from(45.0_f64);
    let t1976 = t1444 * t835 / F::cast_from(45.0_f64);
    let t1977 = t1450 * t834;
    (t1974, t1976, t1977)
}
