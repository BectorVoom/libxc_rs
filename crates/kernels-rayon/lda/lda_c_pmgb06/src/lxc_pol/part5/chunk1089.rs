//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1089/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1089(t460: f64, t7465: f64, t6705: f64, t815: f64, t1874: f64, t2592: f64, t16442: f64, t16444: f64, t16448: f64, t16455: f64, t6626: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20107 = t7465 * t460 / 30.0_f64;
    let t20109 = t6705 * t815 / 10.0_f64;
    let t20111 = t2592 * t1874 / 10.0_f64;
    let t20112 = t16442 / 15.0_f64;
    let t20113 = t16444 / 15.0_f64;
    let t20115 = 2.0_f64 / 15.0_f64 * t16448;
    let t20116 = 2.0_f64 / 15.0_f64 * t16455;
    let t20120 = t802 * t6626;
    (t20107, t20109, t20111, t20112, t20113, t20115, t20116, t20120)
}
