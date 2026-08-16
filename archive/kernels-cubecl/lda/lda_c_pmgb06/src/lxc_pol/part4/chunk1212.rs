//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1212/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1212<F: Float>(t10288: F, t439: F, t6523: F, t1444: F, t6518: F, t1382: F, t6134: F, t11914: F, t11917: F, t2948: F, t6364: F, t2010: F, t6371: F) -> (F, F, F, F, F, F, F) {
    let t15978 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t10288 * t6523;
    let t15980 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1444 * t6518;
    let t15982 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6134 * t1382;
    let t15983 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t11914;
    let t15984 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t11917;
    let t15987 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t2948 * t6364;
    let t15990 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2010 * t2948 * t6371;
    (t15978, t15980, t15982, t15983, t15984, t15987, t15990)
}
