//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 319/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk319<F: Float>(t1138: F, t20: F, t161: F, t635: F, t1129: F, t1132: F, t1135: F, t628: F, t629: F) -> (F, F, F) {
    let t1139 = t1138 * t20;
    let t1140 = t635 * t161;
    let t1143 = t1129 / F::cast_from(2.0_f64) + F::cast_from(0.0627_f64) * t1132 * t629 - F::cast_from(0.0418_f64) * t628 * t1135 + F::cast_from(0.00786258_f64) * t1139 * t1140;
    (t1139, t1140, t1143)
}
