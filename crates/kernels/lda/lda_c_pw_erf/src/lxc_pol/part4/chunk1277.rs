//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1277/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1277<F: Float>(t18956: F, t18957: F, t18959: F, t18960: F, t18963: F, t18978: F, t18990: F, t19002: F, t2357: F, t39: F, t10878: F, t145: F, t15237: F, t15241: F, t15244: F, t15247: F, t15250: F, t15253: F, t15256: F, t15259: F, t15266: F, t15270: F, t15272: F, t15274: F, t169: F, t17095: F, t171: F, t18942: F, t18945: F, t242: F) -> (F, F) {
    let t19005 = t18956 + t18957 + t18959 + t18960 + t18963 + t18978 + t18990 + t19002;
    let t19008 = t39 * t2357;
    let t19013 = -0.031835665774679375 * t169 * t171 * t17095 * t242 - 0.06367133154935875 * t18942 - 0.031835665774679375 * t18945 + 1.0376068845080684 * t15237 + 0.10611888591559791 * t15241 + 0.42447554366239165 * t15244 + 0.3183566577467937 * t15247 - 0.1273426630987175 * t15250 - 0.06367133154935875 * t15253 - 0.5659673915498555 * t15256 - 0.8489510873247833 * t15259 - 0.06367133154935875 * t15266 + 0.05332506774217938 * t145 * t19005 + t10878 + 0.31995040645307626 * t19008 - 2.55960325162461 * t15270 - 0.2133002709687175 * t15272 + 1.279801625812305 * t15274;
    (t19005, t19013)
}
