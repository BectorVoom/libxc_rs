//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 877/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk877<F: Float>(t3148: F, t335: F, t913: F, t904: F, t914: F, t935: F, t3115: F, t3136: F, t905: F, t1081: F, t3001: F, t987: F) -> (F, F, F, F, F) {
    let t8536 = F::new(8.0) * t913 * t335 * t3148;
    let t8539 = F::new(36.0) * t904 * t914 * t935;
    let t8542 = F::cast_from(578.9456755974397_f64) * t3136 * t3115 * t905;
    let t8545 = t3001 * t1081;
    let t8561 = t987 * t987;
    (t8536, t8539, t8542, t8545, t8561)
}
