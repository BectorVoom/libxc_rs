//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 599/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk599<F: Float>(t198: F, t2070: F, t185: F, t1333: F, t212: F) -> (F, F, F) {
    let t4039 = t2070 * t198;
    let t4041 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t185 * t4039;
    let t4048 = F::cast_from(1.0_f64) / t212 / t1333;
    (t4039, t4041, t4048)
}
