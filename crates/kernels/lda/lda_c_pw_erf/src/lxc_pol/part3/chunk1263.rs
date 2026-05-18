//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1263/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1263<F: Float>(t1896: F, t646: F, t12088: F, t12092: F, t12096: F, t12098: F, t12100: F, t12101: F, t12102: F, t12103: F, t12104: F, t12105: F, t12108: F, t12112: F, t12117: F) -> F {
    let t14992 = t1896 * t646;
    let t14994 = t12088 + t12092 + t12096 - t12098 + t12100 + t12101 - t12102 + t12103 + t12104 + t12105 - F::new(0.06649088888888889) * t14992 + t12108 - t12112 - t12117;
    t14994
}
