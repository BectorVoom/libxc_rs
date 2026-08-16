//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1255/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1255(t3785: f64, t7810: f64, t1166: f64, t9593: f64, t2828: f64, t536: f64, t7744: f64, t3788: f64, t9597: f64, t1117: f64, t9602: f64, t2867: f64, t9548: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26194 = t3785 * t7810;
    let t26226 = t1166 * t9593;
    let t26231 = t536 * t2828 * t7744;
    let t26333 = t3788 * t9597;
    let t26345 = t1117 * t9602;
    let t26403 = t2867 * t9548;
    (t26194, t26226, t26231, t26333, t26345, t26403)
}
