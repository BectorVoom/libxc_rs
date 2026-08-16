//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 49/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk49<F: Float>(t95: F, t102: F) -> (F, F, F) {
    let t103 = F::cast_from(1.0_f64) / t95;
    let t105 = F::cast_from(2.923025_f64) * t102 * t103;
    let t107 = pow_1_3::<F>(F::cast_from(9.0_f64));
    let t108 = t107 * t107;
    (t103, t105, t108)
}
