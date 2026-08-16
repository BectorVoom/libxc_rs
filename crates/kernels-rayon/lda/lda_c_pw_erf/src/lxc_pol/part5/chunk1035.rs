//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1035/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1035(t2497: f64, t3966: f64, t10030: f64, t6725: f64, t1519: f64, t2443: f64, t2137: f64, t6851: f64, t519: f64, t5237: f64, t6352: f64, t3863: f64, t571: f64, t6356: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18188 = t3966 * t2497;
    let t18192 = t10030 * t6725;
    let t18280 = t2443 * t1519;
    let t18292 = t6851 * t2137;
    let t18308 = t519 * t5237 * t6352;
    let t18311 = t571 * t3863 * t6356;
    (t18188, t18192, t18280, t18292, t18308, t18311)
}
