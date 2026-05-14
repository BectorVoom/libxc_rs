//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 998/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk998<F: Float>(t2014: F, t3742: F, t3854: F, t4684: F, t571: F, t2967: F, t4670: F, t1319: F, t2023: F, t4624: F, t519: F, t5237: F, t1620: F, t838: F, t1931: F, t610: F) -> (F, F, F, F, F, F, F, F) {
    let t13364 = 16.0 / 15.0 * t3742 * t2014;
    let t13366 = t571 * t3854 * t4684;
    let t13367 = 16.0 / 15.0 * t13366;
    let t13368 = t4670 * t2967;
    let t13371 = 32.0 / 15.0 * t571 * t1319 * t13368;
    let t13373 = 8.0 / 15.0 * t3742 * t2023;
    let t13375 = t519 * t5237 * t4624;
    let t13376 = 8.0 / 27.0 * t13375;
    let t13377 = t838 * t1620;
    let t13379 = t1931 * t610;
    (t13364, t13367, t13368, t13371, t13373, t13376, t13377, t13379)
}
