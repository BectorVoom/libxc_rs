//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 850/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk850<F: Float>(t4459: F, t5727: F, t5743: F, t5942: F, t312: F, t19: F, t2316: F, t729: F, t734: F, t1729: F, t454: F, t776: F) -> (F, F, F, F, F) {
    let t5944 = t4459 + t5727 + t5743 + t5942;
    let t5945 = t5944 * t312;
    let t5949 = t2316 * t729 * t19;
    let t5950 = t5949 * t734;
    let t6025 = t1729 * t776 * t454;
    (t5944, t5945, t5949, t5950, t6025)
}
