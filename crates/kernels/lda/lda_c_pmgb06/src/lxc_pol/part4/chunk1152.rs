//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1152/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1152<F: Float>(t1602: F, t2549: F, t2871: F, t493: F, t11877: F, t5336: F, t4861: F, t6747: F, t1447: F, t6744: F, t6748: F, t6791: F) -> (F, F, F, F, F, F) {
    let t15173 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t2871 * t2549 * t1602;
    let t15176 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t493 * t11877 * t5336;
    let t15179 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t493 * t6747 * t4861;
    let t15180 = t1447 * t6744;
    let t15181 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t15180;
    let t15182 = t1447 * t6748;
    let t15183 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t15182;
    let t15184 = t1447 * t6791;
    (t15173, t15176, t15179, t15181, t15183, t15184)
}
