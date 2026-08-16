//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 397/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk397(t1028: f64, t1050: f64, t1054: f64, t1059: f64, t1102: f64, t1109: f64, t1114: f64, t1386: f64, t1449: f64, t1523: f64, t1524: f64, t462: f64, t493: f64) -> f64 {
    let t1528 = t1028 + t1050 - t1054 - t1059 + t462 * t1524 + t1102 + 0.19751673498613801407e-1_f64 * t1523 * t493 - t1109 - t1114 - t1386 - t1449;
    t1528
}
