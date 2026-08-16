//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 537/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk537(t749: f64, t754: f64, t97: f64, t1786: f64, t27: f64, t321: f64, t106: f64, t32: f64, t1179: f64, t295: f64, t315: f64, t52: f64, t934: f64) -> (f64, f64, f64, f64, f64) {
    let t2760 = t749 * t754 * t97;
    let t2764 = t321 * t1786 * t27;
    let t2765 = t106 * t32;
    let t2767 = t2765 * t1179 * t295;
    let t2771 = t934 * t315 * t52;
    (t2760, t2764, t2765, t2767, t2771)
}
