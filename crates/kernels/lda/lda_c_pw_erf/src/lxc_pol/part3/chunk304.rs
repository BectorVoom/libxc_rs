//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 304/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk304<F: Float>(t1012: F, t386: F, t920: F, t923: F, t925: F, t929: F, t931: F, t933: F) -> (F, F) {
    let t1013 = t1012 * t386;
    let t1022 = -F::cast_from(0.5753888888888888_f64) * t920 + F::cast_from(1.1507777777777777_f64) * t923 + F::cast_from(0.4025666666666667_f64) * t925 + F::new(0.0366775) * t929 + F::new(0.073355) * t931 + F::new(0.137975) * t933;
    (t1013, t1022)
}
