//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1155/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1155(t1988: f64, t493: f64, t6544: f64, t1444: f64, t7634: f64, t1420: f64, t7532: f64, t12772: f64, t2500: f64, t439: f64, t13709: f64, t13714: f64, t13720: f64, t20863: f64, t20866: f64, t20870: f64, t20872: f64, t20874: f64) -> (f64, f64, f64, f64, f64) {
    let t20877 = t493 * t1988 * t6544 / 15.0_f64;
    let t20879 = t1444 * t7634 / 9.0_f64;
    let t20881 = 2.0_f64 / 15.0_f64 * t1420 * t7532;
    let t20884 = 2.0_f64 / 15.0_f64 * t439 * t12772 * t2500;
    let t20885 = t20863 + t20866 + t20870 - t13709 - t13714 - t13720 + t20872 + t20874 + t20877 + t20879 - t20881 - t20884;
    (t20877, t20879, t20881, t20884, t20885)
}
