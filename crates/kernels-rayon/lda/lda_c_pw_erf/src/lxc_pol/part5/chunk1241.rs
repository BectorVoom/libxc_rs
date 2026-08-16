//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1241/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1241(t2171: f64, t6696: f64, t1475: f64, t571: f64, t7608: f64, t10527: f64, t7612: f64, t1318: f64, t34: f64, t4892: f64, t6963: f64, t3667: f64, t7513: f64) -> (f64, f64, f64, f64, f64) {
    let t22306 = t2171 * t6696;
    let t22307 = 8.0_f64 / 45.0_f64 * t22306;
    let t22309 = t571 * t1475 * t7608;
    let t22310 = 8.0_f64 / 135.0_f64 * t22309;
    let t22312 = t571 * t10527 * t7612;
    let t22313 = 64.0_f64 / 243.0_f64 * t22312;
    let t22317 = 8.0_f64 / 5.0_f64 * t1318 * t4892 * t6963 * t34;
    let t22318 = t3667 * t7513;
    (t22307, t22310, t22313, t22317, t22318)
}
