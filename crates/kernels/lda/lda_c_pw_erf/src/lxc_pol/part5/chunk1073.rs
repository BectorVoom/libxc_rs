//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1073/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1073<F: Float>(t22309: F, t10527: F, t571: F, t7612: F, t1318: F, t34: F, t4892: F, t6963: F, t3667: F, t7513: F, t1466: F, t549: F, t593: F, t9237: F, t1325: F, t3787: F, t7588: F) -> (F, F, F, F, F, F) {
    let t22310 = 8.0 / 135.0 * t22309;
    let t22312 = t571 * t10527 * t7612;
    let t22313 = 64.0 / 243.0 * t22312;
    let t22317 = 8.0 / 5.0 * t1318 * t4892 * t6963 * t34;
    let t22318 = t3667 * t7513;
    let t22322 = 8.0 / 5.0 * t1318 * t1466 * t22318 * t549;
    let t22327 = 16.0 / 5.0 * t571 * t1466 * t9237 * t7513 * t593;
    let t22329 = t1325 * t3787 * t7588;
    (t22310, t22313, t22317, t22322, t22327, t22329)
}
