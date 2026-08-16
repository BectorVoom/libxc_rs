//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1111/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1111(t10888: f64, t10890: f64, t10893: f64, t10898: f64, t10913: f64, t10915: f64, t10922: f64, t10924: f64, t6969: f64, t7082: f64, t9008: f64, t9161: f64) -> f64 {
    let t10926 = 0.142419375e1_f64 * t10888 - 0.1898925e1_f64 * t10890 - 0.9494625e0_f64 * t10893 + 0.1898925e1_f64 * t10915 - t7082 + 0.39862222222222222223e0_f64 * t6969 + 0.79724444444444444445e0_f64 * t9008 - t9161 - 0.29896666666666666667e0_f64 * t10898 + 0.8969e0_f64 * t10913 - 0.76790625e-1_f64 * t10922 + 0.3071625e0_f64 * t10924;
    t10926
}
