//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 842/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk842(t1786: f64, t27: f64, t2767: f64, t749: f64, t2760: f64, t2771: f64, t312: f64, t321: f64, t4343: f64, t642: f64, t1767: f64, t2764: f64, t2765: f64, t295: f64) -> (f64, f64, f64, f64) {
    let t8032 = t749 * t1786 * t27 * t2767;
    let t8034 = t2760 * t2771;
    let t8039 = 2.8440036129162336_f64 * t321 * t4343 * t642 * t312;
    let t8043 = 3.8666484793229623_f64 * t2764 * t2765 * t1767 * t295;
    (t8032, t8034, t8039, t8043)
}
