//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 967/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk967(t2760: f64, t2771: f64, t312: f64, t321: f64, t4343: f64, t642: f64, t1767: f64, t2764: f64, t2765: f64, t295: f64, t52: f64, t740: f64, t933: f64, t934: f64) -> (f64, f64, f64, f64) {
    let t8034 = t2760 * t2771;
    let t8039 = 2.8440036129162336_f64 * t321 * t4343 * t642 * t312;
    let t8043 = 3.8666484793229623_f64 * t2764 * t2765 * t1767 * t295;
    let t8047 = 0.6085382050380247_f64 * t933 * t934 * t740 * t52;
    (t8034, t8039, t8043, t8047)
}
