//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 297/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk297<F: Float>(t371: F, t973: F, t920: F, t923: F, t925: F, t929: F, t931: F, t933: F) -> (F, F) {
    let t974 = t973 * t371;
    let t983 = -F::cast_from(0.7843833333333333_f64) * t920 + F::cast_from(1.5687666666666666_f64) * t923 + F::cast_from(0.6886333333333333_f64) * t925 + F::cast_from(0.14025833333333335_f64) * t929 + F::cast_from(0.2805166666666667_f64) * t931 + F::cast_from(0.17365833333333333_f64) * t933;
    (t974, t983)
}
