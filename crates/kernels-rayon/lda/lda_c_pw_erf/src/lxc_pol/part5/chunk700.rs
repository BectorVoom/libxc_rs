//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 700/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk700(t2396: f64, t3802: f64, t519: f64, t2388: f64, t3863: f64, t571: f64, t2384: f64, t3854: f64, t1318: f64, t811: f64, t833: f64, t593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6233 = t3802 * t2396;
    let t6234 = t519 * t6233;
    let t6235 = 16.0_f64 / 135.0_f64 * t6234;
    let t6236 = t3863 * t2388;
    let t6237 = t571 * t6236;
    let t6238 = 16.0_f64 / 135.0_f64 * t6237;
    let t6239 = t3854 * t2384;
    let t6240 = t1318 * t6239;
    let t6241 = 32.0_f64 / 135.0_f64 * t6240;
    let t6242 = t811 * t833;
    let t6243 = t6242 * t593;
    (t6233, t6234, t6235, t6236, t6237, t6238, t6239, t6240, t6241, t6242, t6243)
}
