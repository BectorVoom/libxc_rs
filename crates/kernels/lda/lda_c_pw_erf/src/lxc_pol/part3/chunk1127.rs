//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1127/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1127<F: Float>(t10881: F, t10883: F, t10886: F, t10893: F, t10897: F, t10900: F, t10903: F, t10906: F, t10909: F, t10913: F, t10915: F, t10918: F, t10922: F, t10956: F, t10957: F, t10963: F) -> (F,) {
    let t15234 = -3.839404877436915 * t10963 - 0.10665013548435875 * t10957 + t10897 + 0.053059442957798957 * t10900 + 0.3183566577467937 * t10903 + 0.4775349866201906 * t10906 + 1.5564103267621028 * t10909 + t10913 - 0.42447554366239165 * t10915 - 1.273426630987175 * t10918 - t10922 - t10956 - t10881 - 0.09550699732403813 * t10883 - 0.09550699732403813 * t10886 - 0.031835665774679375 * t10893;
    (t15234,)
}
