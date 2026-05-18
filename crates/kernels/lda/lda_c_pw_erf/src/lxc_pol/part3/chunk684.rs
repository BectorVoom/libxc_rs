//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 684/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk684<F: Float>(t1619: F, t225: F, t10: F, t602: F, t1634: F, t245: F, t1638: F, t1627: F, t1631: F, t156: F, t603: F, t635: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4195 = t225 * t1619;
    let t4196 = t10 * t4195;
    let t4198 = F::new(0.3246312408709453) * t602 * t4196;
    let t4199 = t245 * t1634;
    let t4201 = F::new(0.03354522822333102) * t1638 * t4199;
    let t4202 = t1631 * t1627;
    let t4204 = t156 * t1634;
    let t4206 = F::new(0.21642082724729686) * t602 * t4204;
    let t4207 = t635 * t603;
    (t4195, t4196, t4198, t4199, t4201, t4202, t4204, t4206, t4207)
}
