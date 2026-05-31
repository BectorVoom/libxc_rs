//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1314/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1314<F: Float>(t4880: F, t493: F, t6751: F, t13483: F, t176: F, t4885: F, t1981: F, t4866: F, t1447: F, t6756: F, t6761: F, t6766: F) -> (F, F, F, F, F, F) {
    let t17275 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t493 * t6751 * t4880;
    let t17276 = t13483 * t176;
    let t17279 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t493 * t17276 * t4885;
    let t17282 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1981 * t6751 * t4866;
    let t17283 = t1447 * t6756;
    let t17284 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t17283;
    let t17285 = t1447 * t6761;
    let t17286 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t17285;
    let t17287 = t1447 * t6766;
    (t17275, t17279, t17282, t17284, t17286, t17287)
}
