//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1147/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1147<F: Float>(t1972: F, t6292: F, t1444: F, t7656: F, t2488: F, t493: F, t5312: F, t1420: F, t7646: F, t17577: F, t432: F, t7803: F) -> (F, F, F, F, F, F) {
    let t20784 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t6292;
    let t20786 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1444 * t7656;
    let t20789 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t5312 * t2488;
    let t20791 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1420 * t7646;
    let t20792 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t17577;
    let t20794 = t432 * t7803 / F::cast_from(30.0_f64);
    (t20784, t20786, t20789, t20791, t20792, t20794)
}
