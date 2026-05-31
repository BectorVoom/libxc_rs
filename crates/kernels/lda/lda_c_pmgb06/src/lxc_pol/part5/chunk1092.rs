//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1092/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1092<F: Float>(t6227: F, t802: F, t16542: F, t16549: F, t20127: F, t20129: F, t20131: F, t20133: F, t20135: F, t20138: F, t20139: F, t20140: F) -> (F, F, F, F) {
    let t20142 = t802 * t6227 / F::cast_from(10.0_f64);
    let t20143 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t16542;
    let t20144 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16549;
    let t20145 = -t20127 - t20129 + t20131 + t20133 + t20135 + t20138 - t20139 - t20140 - t20142 - t20143 + t20144;
    (t20142, t20143, t20144, t20145)
}
