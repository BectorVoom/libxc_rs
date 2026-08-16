//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1323/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1323(t10881: f64, t10883: f64, t10886: f64, t10893: f64, t10897: f64, t10900: f64, t10903: f64, t10906: f64, t10909: f64, t10913: f64, t10915: f64, t10918: f64, t10922: f64, t10956: f64, t10957: f64, t10963: f64) -> f64 {
    let t15234 = -3.839404877436915_f64 * t10963 - 0.10665013548435875_f64 * t10957 + t10897 + 0.053059442957798957_f64 * t10900 + 0.3183566577467937_f64 * t10903 + 0.4775349866201906_f64 * t10906 + 1.5564103267621028_f64 * t10909 + t10913 - 0.42447554366239165_f64 * t10915 - 1.273426630987175_f64 * t10918 - t10922 - t10956 - t10881 - 0.09550699732403813_f64 * t10883 - 0.09550699732403813_f64 * t10886 - 0.031835665774679375_f64 * t10893;
    t15234
}
