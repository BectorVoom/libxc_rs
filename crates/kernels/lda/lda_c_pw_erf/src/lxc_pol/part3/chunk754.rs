//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 754/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk754<F: Float>(t219: F, t4867: F, t4676: F, t571: F, t2021: F, t954: F, t1308: F, t2193: F, t3416: F, t1450: F, t2171: F, t2098: F, t529: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4868 = t4867 * t219;
    let t4869 = t4868 * t4676;
    let t4871 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t571 * t4869;
    let t4872 = t2021 * t954;
    let t4873 = t1308 * t4872;
    let t4875 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t571 * t4873;
    let t4877 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3416 * t2193;
    let t4879 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t2171 * t1450;
    let t4880 = t529 * t2098;
    (t4868, t4869, t4871, t4872, t4873, t4875, t4877, t4879, t4880)
}
