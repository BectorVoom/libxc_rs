//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1016/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1016<F: Float>(t2505: F, t5215: F, t12357: F, t15015: F, t21269: F, t21271: F, t21274: F, t21276: F, t21277: F, t21278: F, t21279: F, t21281: F, t21283: F, t21285: F, t21287: F, t2100: F, t2407: F) -> (F, F, F) {
    let t21289 = 4.0 / 5.0 * t5215 * t2505;
    let t21290 = -t21269 + t12357 + t21271 + 0.004546314527777778 * t15015 - t21274 + t21276 - t21277 + t21278 + t21279 + t21281 + t21283 + t21285 - t21287 + t21289;
    let t21294 = 4.0 / 5.0 * t2407 * t2100;
    (t21289, t21290, t21294)
}
