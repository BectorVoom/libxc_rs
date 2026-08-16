//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 622/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk622(t1193: f64, t1354: f64, t2822: f64, t1166: f64, t409: f64, t419: f64, t421: f64, t1186: f64, t1343: f64, t398: f64, t740: f64) -> (f64, f64, f64, f64, f64) {
    let t2825 = 0.0034679929861433484_f64 * t2822 * t1193 * t1354;
    let t2826 = t409 * t1166;
    let t2828 = t2826 * t419 * t421;
    let t2831 = t1343 * t1186 * t421;
    let t2833 = t740 * t398;
    (t2825, t2826, t2828, t2831, t2833)
}
