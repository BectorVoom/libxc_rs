//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1214/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1214(t2171: f64, t6282: f64, t6428: f64, t2031: f64, t6988: f64, t1987: f64, t1992: f64, t3863: f64, t571: f64, t7815: f64, t4763: f64, t6239: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21915 = 4.0_f64 / 15.0_f64 * t2171 * t6282;
    let t21917 = 8.0_f64 / 15.0_f64 * t2171 * t6428;
    let t21919 = 8.0_f64 / 15.0_f64 * t6988 * t2031;
    let t21921 = 16.0_f64 / 15.0_f64 * t6988 * t1987;
    let t21923 = 8.0_f64 / 9.0_f64 * t6988 * t1992;
    let t21925 = t571 * t3863 * t7815;
    let t21926 = 8.0_f64 / 45.0_f64 * t21925;
    let t21927 = t4763 * t6239;
    (t21915, t21917, t21919, t21921, t21923, t21926, t21927)
}
