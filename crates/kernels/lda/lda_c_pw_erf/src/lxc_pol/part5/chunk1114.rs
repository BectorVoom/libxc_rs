//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1114/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1114<F: Float>(t153: F, t474: F, t7387: F, t11219: F, t11222: F, t11229: F, t11232: F, t11233: F, t11236: F, t14933: F, t14935: F, t14942: F, t14943: F, t14948: F, t14950: F, t14957: F, t14958: F, t14961: F, t14965: F, t19385: F, t19388: F, t19397: F) -> (F,) {
    let t23102 = t153 * t474 * t7387;
    let t23115 = -0.5694518669548363 * t23102 + t14933 - 13.28721022894618 * t14935 - t14942 - 1.5077307696390791 * t14943 + t14948 + 0.7538653848195396 * t14950 - t14957 - 0.2512884616065132 * t14958 - t14961 - 0.2512884616065132 * t19385 - 0.2512884616065132 * t19388 + 1.5077307696390791 * t14965 - 0.5025769232130264 * t11219 - t11222 - 0.0837628205355044 * t11229 - t11232 + 0.2512884616065132 * t11233 + t11236 - 0.5025769232130264 * t19397;
    (t23115,)
}
