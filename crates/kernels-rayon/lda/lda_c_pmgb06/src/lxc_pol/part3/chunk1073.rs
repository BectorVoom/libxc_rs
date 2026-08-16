//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1073/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1073(t1069: f64, t1438: f64, t2064: f64, t2960: f64, t439: f64, t1908: f64, t3213: f64, t12719: f64, t12724: f64, t12726: f64, t12728: f64, t12730: f64, t12733: f64, t12737: f64, t12741: f64, t12743: f64, t12746: f64) -> (f64, f64, f64) {
    let t12751 = t439 * t2960 * t2064 * t1438 * t1069 / 9.0_f64;
    let t12752 = t3213 * t1908;
    let t12753 = 2.0_f64 / 135.0_f64 * t12752;
    let t12754 = -t12719 - t12724 - t12726 - t12728 - t12730 - t12733 - t12737 - t12741 - t12743 - t12746 - t12751 + t12753;
    (t12751, t12753, t12754)
}
