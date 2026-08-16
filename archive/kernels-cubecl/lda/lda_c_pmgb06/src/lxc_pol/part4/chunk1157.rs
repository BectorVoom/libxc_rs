//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1157/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1157<F: Float>(t1080: F, t6502: F, t1915: F, t493: F, t1602: F, t2545: F, t2871: F, t12633: F, t439: F, t5364: F, t5344: F, t5482: F) -> (F, F, F, F, F) {
    let t15223 = t6502 * t1080;
    let t15226 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t493 * t1915 * t15223;
    let t15230 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t493 * t2871 * t2545 * t1602;
    let t15233 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t12633 * t5364;
    let t15236 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t5482 * t5344;
    (t15223, t15226, t15230, t15233, t15236)
}
