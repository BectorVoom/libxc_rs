//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 644/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk644(t1211: f64, t1223: f64, t1959: f64, t3068: f64, t3072: f64, t3073: f64, t3076: f64, t3105: f64, t616: f64, t618: f64, t632: f64, t72: f64, t85: f64) -> f64 {
    let t3108 = -t3072 * t3073 / 2.0_f64 + 2.0_f64 * t1959 * t3076 - t618 * t3068 + 2.0_f64 * t3068 * t85 + 2.0_f64 * t1211 * t632 + 2.0_f64 * t616 * t1223 + 2.0_f64 * t72 * t3105;
    t3108
}
