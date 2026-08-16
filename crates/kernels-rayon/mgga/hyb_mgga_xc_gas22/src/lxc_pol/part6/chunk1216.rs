//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1216/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1216(t13: f64, t20162: f64, t2969: f64, t1806: f64, t2970: f64, t2974: f64, t800: f64, t92: f64, t7842: f64, t7843: f64, t7848: f64, t639: f64, t7847: f64) -> (f64, f64, f64, f64) {
    let t23667 = t20162 * t13 * t2969;
    let t23674 = t2970 * t800 * t1806 * t92 * t2974;
    let t23684 = t7842 * t7848 * t7843;
    let t23688 = t2970 * t7847 * t639 * t2974;
    (t23667, t23674, t23684, t23688)
}
