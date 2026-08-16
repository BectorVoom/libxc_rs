//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1314/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1314(t4880: f64, t493: f64, t6751: f64, t13483: f64, t176: f64, t4885: f64, t1981: f64, t4866: f64, t1447: f64, t6756: f64, t6761: f64, t6766: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17275 = 2.0_f64 / 27.0_f64 * t493 * t6751 * t4880;
    let t17276 = t13483 * t176;
    let t17279 = 16.0_f64 / 81.0_f64 * t493 * t17276 * t4885;
    let t17282 = 8.0_f64 / 27.0_f64 * t1981 * t6751 * t4866;
    let t17283 = t1447 * t6756;
    let t17284 = 4.0_f64 / 135.0_f64 * t17283;
    let t17285 = t1447 * t6761;
    let t17286 = 8.0_f64 / 135.0_f64 * t17285;
    let t17287 = t1447 * t6766;
    (t17275, t17279, t17282, t17284, t17286, t17287)
}
