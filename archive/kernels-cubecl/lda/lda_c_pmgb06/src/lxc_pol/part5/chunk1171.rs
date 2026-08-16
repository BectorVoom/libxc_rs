//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1171/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1171<F: Float>(t17886: F, t17890: F, t1444: F, t7715: F, t2979: F, t493: F, t7714: F, t1380: F, t2088: F, t2545: F, t1423: F, t7525: F) -> (F, F, F, F, F, F) {
    let t21068 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t17886;
    let t21069 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t17890;
    let t21071 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1444 * t7715;
    let t21074 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t2979 * t7714;
    let t21078 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t1380 * t2545 * t2088;
    let t21079 = t1423 * t7525;
    (t21068, t21069, t21071, t21074, t21078, t21079)
}
