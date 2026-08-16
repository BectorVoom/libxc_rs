//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1162/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1162<F: Float>(t2979: F, t4731: F, t493: F, t1981: F, t5441: F, t1380: F, t3382: F, t838: F, t1912: F, t3226: F, t1447: F, t4728: F) -> (F, F, F, F, F) {
    let t13875 = t493 * t2979 * t4731 / F::cast_from(15.0_f64);
    let t13878 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1981 * t2979 * t5441;
    let t13882 = t493 * t1380 * t838 * t3382 / F::cast_from(45.0_f64);
    let t13883 = t3226 * t1912;
    let t13884 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13883;
    let t13885 = t1447 * t4728;
    (t13875, t13878, t13882, t13884, t13885)
}
