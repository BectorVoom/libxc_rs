//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1161/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1161<F: Float>(t1380: F, t1586: F, t1831: F, t1981: F, t1912: F, t3198: F, t1444: F, t4728: F, t4732: F, t4602: F, t5442: F, t1911: F, t493: F, t9925: F) -> (F, F, F, F, F, F) {
    let t13861 = F::new(2.0) / F::new(15.0) * t1981 * t1380 * t1831 * t1586;
    let t13863 = t3198 * t1912 / F::new(15.0);
    let t13865 = F::new(2.0) / F::new(15.0) * t1444 * t4728;
    let t13867 = t1444 * t4732 / F::new(15.0);
    let t13869 = F::new(4.0) / F::new(15.0) * t4602 * t5442;
    let t13872 = t493 * t9925 * t1911 / F::new(15.0);
    (t13861, t13863, t13865, t13867, t13869, t13872)
}
