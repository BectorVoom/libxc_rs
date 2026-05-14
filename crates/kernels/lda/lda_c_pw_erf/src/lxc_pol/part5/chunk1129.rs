//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1129/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1129<F: Float>(t21274: F, t21276: F, t21277: F, t21278: F, t21279: F, t21281: F, t21283: F, t21285: F, t21287: F, t21289: F, t21294: F, t21296: F, t21303: F, t21305: F, t21307: F, t21309: F, t21311: F, t21313: F, t21315: F, t21317: F, t21318: F, t21319: F, t21320: F, t21322: F, t21323: F, t21324: F) -> (F, F) {
    let t23228 = -t21274 + t21276 - t21277 + t21278 + t21279 + t21281 + t21283 + t21285 - t21287 + t21289 - t21294 + t21296 + t21303;
    let t23231 = -t21305 + t21307 - t21309 + t21311 + t21313 + t21315 + t21317 - t21318 + t21319 - t21320 - t21322 - t21323 - t21324;
    (t23228, t23231)
}
