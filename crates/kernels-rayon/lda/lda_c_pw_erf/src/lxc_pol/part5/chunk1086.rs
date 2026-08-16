//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1086/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1086(t20188: f64, t20189: f64, t20191: f64, t20192: f64, t20195: f64, t20196: f64, t20198: f64, t20201: f64, t10881: f64, t10886: f64, t10897: f64, t10906: f64, t10909: f64, t10913: f64, t10918: f64, t10922: f64, t10956: f64, t10963: f64, t145: f64, t169: f64, t171: f64, t18918: f64, t18920: f64, t18923: f64, t20179: f64, t20185: f64, t242: f64) -> (f64, f64) {
    let t20204 = t20188 + t20189 + t20191 + t20192 + t20195 + t20196 + t20198 + t20201;
    let t20211 = -1.279801625812305_f64 * t10963 + 0.15917832887339686_f64 * t18918 - 0.31995040645307626_f64 * t18920 - 0.031835665774679375_f64 * t169 * t171 * t20179 * t242 - 0.031835665774679375_f64 * t20185 - 0.42447554366239165_f64 * t18923 + 0.05332506774217938_f64 * t145 * t20204 + t10897 + 0.15917832887339686_f64 * t10906 + 0.5188034422540342_f64 * t10909 + t10913 - 0.42447554366239165_f64 * t10918 - t10922 - t10956 - t10881 - 0.031835665774679375_f64 * t10886;
    (t20204, t20211)
}
