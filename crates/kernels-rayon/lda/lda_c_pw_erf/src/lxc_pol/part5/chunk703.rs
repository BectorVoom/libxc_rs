//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 703/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk703(t2328: f64, t504: f64, t348: f64, t1326: f64, t1325: f64, t1997: f64, t2171: f64, t2466: f64, t558: f64, t352: f64, t3867: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6263 = t2328 * t504;
    let t6264 = t6263 * t348;
    let t6265 = t1326 * t6264;
    let t6267 = 8.0_f64 / 45.0_f64 * t1325 * t6265;
    let t6269 = 8.0_f64 / 45.0_f64 * t2171 * t1997;
    let t6270 = t2466 * t558;
    let t6271 = t6270 * t352;
    let t6272 = t3867 * t6271;
    let t6274 = 8.0_f64 / 45.0_f64 * t571 * t6272;
    (t6263, t6264, t6265, t6267, t6269, t6270, t6271, t6272, t6274)
}
