//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1110/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1110<F: Float>(t13193: F, t13198: F, t13201: F, t13206: F, t13208: F, t13210: F, t13212: F, t13214: F, t13216: F, t13221: F, t13223: F, t13225: F, t13229: F, t13232: F, t13234: F, t13239: F, t13242: F, t13245: F, t13248: F, t13251: F, t13253: F, t13256: F, t13259: F, t13262: F, t13264: F, t13269: F, t13274: F) -> (F, F) {
    let t15074 = -t13193 - t13198 + t13201 + t13206 + t13208 + t13210 + t13212 - t13214 - t13216 - t13221 + t13223 + t13225 + t13229;
    let t15075 = t13232 - t13234 - t13239 - t13242 + t13245 + t13248 + t13251 - t13253 - t13256 - t13259 - t13262 + t13264 + t13269 + t13274;
    (t15074, t15075)
}
