//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 711/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk711(t1991: f64, t6352: f64, t519: f64, t2419: f64, t593: f64, t1308: f64, t571: f64, t1333: f64, t2337: f64, t352: f64, t1319: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6353 = t1991 * t6352;
    let t6355 = 8.0_f64 / 9.0_f64 * t519 * t6353;
    let t6356 = t2419 * t593;
    let t6357 = t1308 * t6356;
    let t6359 = 4.0_f64 / 45.0_f64 * t571 * t6357;
    let t6360 = t1333 * t2337;
    let t6361 = t6360 * t352;
    let t6362 = t1319 * t6361;
    let t6364 = 8.0_f64 / 45.0_f64 * t571 * t6362;
    let t6365 = t1351 * t2337;
    (t6353, t6355, t6356, t6357, t6359, t6360, t6361, t6362, t6364, t6365)
}
