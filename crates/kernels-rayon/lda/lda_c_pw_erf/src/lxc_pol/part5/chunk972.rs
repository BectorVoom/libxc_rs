//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 972/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk972(t13924: f64, t2162: f64, t571: f64, t9432: f64, t1351: f64, t4574: f64, t3975: f64, t1518: f64, t185: f64, t2099: f64, t4500: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13925 = 8.0_f64 / 45.0_f64 * t13924;
    let t13929 = t571 * t9432 * t2162;
    let t13930 = 8.0_f64 / 45.0_f64 * t13929;
    let t13962 = t4574 * t1351;
    let t13966 = t3975 * t1351;
    let t14004 = t185 * t1518 * t2099;
    let t14005 = 4.0_f64 / 45.0_f64 * t14004;
    let t14014 = t4500 * t784;
    (t13925, t13930, t13962, t13966, t14005, t14014)
}
