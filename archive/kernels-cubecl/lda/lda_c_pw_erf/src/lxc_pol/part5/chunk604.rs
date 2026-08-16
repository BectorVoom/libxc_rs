//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 604/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk604<F: Float>(t2910: F, t482: F, t485: F, t1098: F, t1597: F, t2916: F, t1186: F, t1124: F, t465: F, t483: F, t1131: F, t1578: F) -> (F, F, F, F, F, F) {
    let t4160 = F::cast_from(0.005926167098672845_f64) * t482 * t2910 * t485;
    let t4163 = F::cast_from(0.0014862827083471494_f64) * t1098 * t2916 * t1597;
    let t4165 = F::cast_from(0.025899545097903542_f64) * t1186 * t485;
    let t4166 = t1124 * t465;
    let t4168 = t4166 * t483 * t485;
    let t4172 = F::cast_from(0.01975389032890948_f64) * t1578 * t1131 * t485;
    (t4160, t4163, t4165, t4166, t4168, t4172)
}
