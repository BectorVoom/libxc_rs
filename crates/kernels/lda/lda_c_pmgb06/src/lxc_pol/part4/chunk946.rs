//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 946/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk946<F: Float>(t1105: F, t2158: F, t2148: F, t3729: F, t27: F, t4515: F, t693: F, t3725: F, t1108: F, t2160: F, t1112: F, t4529: F, t2151: F, t3734: F, t4556: F, t980: F) -> (F, F, F, F, F, F, F, F) {
    let t11139 = t1105 * t2158;
    let t11142 = t2148 * t3729;
    let t11145 = t4515 * t27 * t693;
    let t11147 = t2148 * t3725;
    let t11149 = t1108 * t2160;
    let t11155 = t4529 * t1112;
    let t11157 = t2151 * t3734;
    let t11160 = t4556 * t980;
    (t11139, t11142, t11145, t11147, t11149, t11155, t11157, t11160)
}
