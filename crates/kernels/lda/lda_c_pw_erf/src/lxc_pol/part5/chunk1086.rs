//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1086/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1086<F: Float>(t20188: F, t20189: F, t20191: F, t20192: F, t20195: F, t20196: F, t20198: F, t20201: F, t10881: F, t10886: F, t10897: F, t10906: F, t10909: F, t10913: F, t10918: F, t10922: F, t10956: F, t10963: F, t145: F, t169: F, t171: F, t18918: F, t18920: F, t18923: F, t20179: F, t20185: F, t242: F) -> (F, F) {
    let t20204 = t20188 + t20189 + t20191 + t20192 + t20195 + t20196 + t20198 + t20201;
    let t20211 = -F::cast_from(1.279801625812305_f64) * t10963 + F::cast_from(0.15917832887339686_f64) * t18918 - F::cast_from(0.31995040645307626_f64) * t18920 - F::cast_from(0.031835665774679375_f64) * t169 * t171 * t20179 * t242 - F::cast_from(0.031835665774679375_f64) * t20185 - F::cast_from(0.42447554366239165_f64) * t18923 + F::cast_from(0.05332506774217938_f64) * t145 * t20204 + t10897 + F::cast_from(0.15917832887339686_f64) * t10906 + F::cast_from(0.5188034422540342_f64) * t10909 + t10913 - F::cast_from(0.42447554366239165_f64) * t10918 - t10922 - t10956 - t10881 - F::cast_from(0.031835665774679375_f64) * t10886;
    (t20204, t20211)
}
