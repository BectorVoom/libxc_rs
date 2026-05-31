//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 286/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk286<F: Float>(t11: F, t19: F, t919: F, t328: F, t922: F, t21: F, t635: F) -> (F, F, F, F) {
    let t927 = F::cast_from(1.0_f64)/F::sqrt(t11);
    let t928 = t927 * t19;
    let t929 = t928 * t919;
    let t931 = t328 * t922;
    let t933 = t21 * t635;
    (t928, t929, t931, t933)
}
