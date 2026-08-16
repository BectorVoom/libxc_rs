//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 704/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk704(t2478: f64, t558: f64, t352: f64, t1319: f64, t1318: f64, t2497: f64, t504: f64, t348: f64, t1313: f64, t519: f64, t2526: f64, t1308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6275 = t2478 * t558;
    let t6276 = t6275 * t352;
    let t6277 = t1319 * t6276;
    let t6279 = 8.0_f64 / 45.0_f64 * t1318 * t6277;
    let t6280 = t2497 * t504;
    let t6281 = t6280 * t348;
    let t6282 = t1313 * t6281;
    let t6284 = 4.0_f64 / 45.0_f64 * t519 * t6282;
    let t6285 = t2526 * t558;
    let t6286 = t6285 * t352;
    let t6287 = t1308 * t6286;
    (t6275, t6276, t6277, t6279, t6280, t6281, t6282, t6284, t6285, t6286, t6287)
}
