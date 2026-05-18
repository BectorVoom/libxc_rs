//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 678/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk678<F: Float>(t1138: F, t1597: F, t2881: F, t2910: F, t482: F, t485: F, t1098: F, t2916: F, t1186: F, t1124: F, t465: F, t483: F) -> (F, F, F, F, F, F) {
    let t4156 = t2881 * t1138 * t1597;
    let t4160 = F::new(0.005926167098672845) * t482 * t2910 * t485;
    let t4163 = F::new(0.0014862827083471494) * t1098 * t2916 * t1597;
    let t4165 = F::new(0.025899545097903542) * t1186 * t485;
    let t4166 = t1124 * t465;
    let t4168 = t4166 * t483 * t485;
    (t4156, t4160, t4163, t4165, t4166, t4168)
}
