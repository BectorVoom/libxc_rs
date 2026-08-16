//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1218/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1218(t3787: f64, t519: f64, t7604: f64, t1325: f64, t7600: f64, t1449: f64, t7474: f64, t2140: f64, t6205: f64, t7007: f64, t3899: f64, t571: f64, t7557: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21964 = t519 * t3787 * t7604;
    let t21965 = 8.0_f64 / 15.0_f64 * t21964;
    let t21967 = t1325 * t3787 * t7600;
    let t21968 = 8.0_f64 / 15.0_f64 * t21967;
    let t21970 = t519 * t1449 * t7474;
    let t21971 = 16.0_f64 / 45.0_f64 * t21970;
    let t21972 = t6205 * t2140;
    let t21973 = 8.0_f64 / 45.0_f64 * t21972;
    let t21974 = t7007 * t2140;
    let t21975 = 16.0_f64 / 45.0_f64 * t21974;
    let t21977 = t571 * t3899 * t7557;
    (t21965, t21968, t21971, t21973, t21975, t21977)
}
