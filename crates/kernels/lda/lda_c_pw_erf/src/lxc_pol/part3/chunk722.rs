//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 722/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk722<F: Float>(t1967: F, t5237: F, t519: F, t1446: F, t2031: F, t1278: F, t789: F, t1313: F, t1991: F, t4624: F, t197: F, t3893: F, t4610: F, t4218: F, t4220: F, t4225: F, t4227: F, t4235: F, t5213: F, t5217: F, t5224: F, t5228: F, t5233: F, t5236: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5238 = t5237 * t1967;
    let t5240 = 16.0 / 81.0 * t519 * t5238;
    let t5242 = 8.0 / 45.0 * t1446 * t2031;
    let t5243 = t789 * t1278;
    let t5244 = t1313 * t5243;
    let t5246 = 4.0 / 45.0 * t519 * t5244;
    let t5247 = t1991 * t4624;
    let t5249 = 4.0 / 27.0 * t519 * t5247;
    let t5250 = t3893 * t197;
    let t5251 = t5250 * t4610;
    let t5253 = 32.0 / 81.0 * t519 * t5251;
    let t5254 = 4.0 / 3.0 * t4218 + 16.0 / 3.0 * t4220 + t5213 + t5217 + 4.0 / 3.0 * t4225 + 8.0 / 3.0 * t4227 + t4235 - t5224 + t5228 + t5233 - t5236 + t5240 - t5242 - t5246 + t5249 + t5253;
    (t5238, t5240, t5242, t5243, t5244, t5246, t5247, t5249, t5250, t5251, t5253, t5254)
}
