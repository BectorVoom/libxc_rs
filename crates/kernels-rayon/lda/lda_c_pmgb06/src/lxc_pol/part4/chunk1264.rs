//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1264/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1264(t12868: f64, t12870: f64, t12878: f64, t132: f64, t435: f64, t6442: f64, t137: f64, t2604: f64, t9610: f64, t1512: f64, t2606: f64, t432: f64, t6443: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16608 = 8.0_f64 / 405.0_f64 * t12868;
    let t16609 = 16.0_f64 / 405.0_f64 * t12870;
    let t16610 = 8.0_f64 / 243.0_f64 * t12878;
    let t16612 = t132 * t435 * t6442;
    let t16613 = 4.0_f64 / 45.0_f64 * t16612;
    let t16617 = t132 * t137 * t9610 * t2604 / 15.0_f64;
    let t16619 = t1512 * t2606 / 15.0_f64;
    let t16621 = 2.0_f64 / 15.0_f64 * t432 * t6443;
    (t16608, t16609, t16610, t16613, t16617, t16619, t16621)
}
