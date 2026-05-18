//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 892/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk892<F: Float>(t3148: F, t335: F, t913: F, t904: F, t914: F, t935: F, t3115: F, t3136: F, t905: F, t987: F, t973: F, t990: F) -> (F, F, F, F, F, F) {
    let t8536 = F::new(8.0) * t913 * t335 * t3148;
    let t8539 = F::new(36.0) * t904 * t914 * t935;
    let t8542 = F::new(578.9456755974397) * t3136 * t3115 * t905;
    let t8561 = t987 * t987;
    let t8564 = t973 * t973;
    let t8565 = t990 * t990;
    (t8536, t8539, t8542, t8561, t8564, t8565)
}
