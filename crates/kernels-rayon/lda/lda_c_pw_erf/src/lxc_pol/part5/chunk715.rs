//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 715/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk715(t1318: f64, t6405: f64, t2415: f64, t593: f64, t1308: f64, t571: f64, t6346: f64, t6350: f64, t6355: f64, t6359: f64, t6364: f64, t6369: f64, t6373: f64, t6377: f64, t6382: f64, t6387: f64, t6391: f64, t6393: f64, t6395: f64, t6399: f64, t6403: f64) -> (f64, f64, f64, f64, f64) {
    let t6407 = 16.0_f64 / 45.0_f64 * t1318 * t6405;
    let t6408 = t2415 * t593;
    let t6409 = t1308 * t6408;
    let t6411 = 8.0_f64 / 45.0_f64 * t571 * t6409;
    let t6412 = t6346 - t6350 - t6355 - t6359 - t6364 + t6369 + t6373 - t6377 - t6382 + t6387 - t6391 + t6393 - t6395 - t6399 + t6403 - t6407 + t6411;
    (t6407, t6408, t6409, t6411, t6412)
}
