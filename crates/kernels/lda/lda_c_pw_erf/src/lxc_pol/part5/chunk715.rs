//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 715/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk715<F: Float>(t1318: F, t6405: F, t2415: F, t593: F, t1308: F, t571: F, t6346: F, t6350: F, t6355: F, t6359: F, t6364: F, t6369: F, t6373: F, t6377: F, t6382: F, t6387: F, t6391: F, t6393: F, t6395: F, t6399: F, t6403: F) -> (F, F, F, F, F) {
    let t6407 = F::new(16.0) / F::new(45.0) * t1318 * t6405;
    let t6408 = t2415 * t593;
    let t6409 = t1308 * t6408;
    let t6411 = F::new(8.0) / F::new(45.0) * t571 * t6409;
    let t6412 = t6346 - t6350 - t6355 - t6359 - t6364 + t6369 + t6373 - t6377 - t6382 + t6387 - t6391 + t6393 - t6395 - t6399 + t6403 - t6407 + t6411;
    (t6407, t6408, t6409, t6411, t6412)
}
