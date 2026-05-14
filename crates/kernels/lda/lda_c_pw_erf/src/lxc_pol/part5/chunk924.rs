//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 924/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk924<F: Float>(t281: F, t285: F, t477: F, t6039: F, t1128: F, t2363: F, t142: F, t6121: F, t455: F, t1549: F, t6097: F, t169: F, t242: F, t299: F, t462: F, t6080: F) -> (F, F, F, F, F, F) {
    let t18888 = t281 * t6039 * t477 * t285;
    let t18892 = t281 * t2363 * t1128 * t285;
    let t18900 = t142 * t6121;
    let t18901 = t455 * t18900;
    let t18906 = t1549 * t6097;
    let t18918 = t169 * t299 * t6039 * t242;
    let t18920 = t462 * t6080;
    (t18888, t18892, t18901, t18906, t18918, t18920)
}
