//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2940/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2940(t41441: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64, t77858: f64) -> f64 {
    let t78088 = 0.19931111111111111111e0_f64 * t77559 - 0.59793333333333333333e0_f64 * t77561 + 0.39862222222222222223e1_f64 * t77566 - 0.99655555555555555554e0_f64 * t77570 - 0.88582716049382716048e0_f64 * t77575 + 0.2434271604938271605e0_f64 * t41441 - 0.39862222222222222223e0_f64 * t63464 + 0.59793333333333333334e0_f64 * t77581 - 0.19931111111111111111e0_f64 * t77586 - 0.71752000000000000002e1_f64 * t77590 + 0.35876000000000000001e1_f64 * t77594 + 0.54771111111111111111e-1_f64 * t77858;
    t78088
}
