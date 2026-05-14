//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 963/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk963<F: Float>(t20127: F, t20129: F, t20131: F, t20133: F, t20135: F, t20138: F, t20139: F, t20140: F, t20142: F, t20143: F, t20144: F, t16556: F, t2386: F, t851: F, t529: F, t13064: F, t5138: F) -> (F, F, F, F, F) {
    let t20145 = -t20127 - t20129 + t20131 + t20133 + t20135 + t20138 - t20139 - t20140 - t20142 - t20143 + t20144;
    let t20146 = 8.0 / 45.0 * t16556;
    let t20147 = t2386 * t851;
    let t20148 = t20147 * t529;
    let t20151 = 2.0 / 9.0 * t5138 * t13064 * t20148;
    (t20145, t20146, t20147, t20148, t20151)
}
