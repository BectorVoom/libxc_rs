//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 745/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk745(t1954: f64, t3881: f64, t1975: f64, t1967: f64, t3876: f64, t623: f64, t627: f64, t74: f64, t79: f64, t81: f64, t82: f64, t1211: f64, t1223: f64, t1959: f64, t618: f64, t72: f64, t85: f64) -> (f64, f64, f64, f64) {
    let t3882 = t1954 * t3881;
    let t3898 = t1975 * t3881;
    let t3909 = -2.0_f64 * t1967 * t3881 * t81 + t623 * t3876 * t81 / 2.0_f64 + t3898 * t81 / 4.0_f64 - 4.0_f64 * t3881 * t82 - t79 * t3881 * t81 - 4.0_f64 * t627 * t3876 - t74 * t3876 * t81;
    let t3912 = -t3882 * t81 / 2.0_f64 + 2.0_f64 * t1959 * t3881 - t618 * t3876 + 2.0_f64 * t3876 * t85 + 4.0_f64 * t1211 * t1223 + 2.0_f64 * t72 * t3909;
    (t3882, t3898, t3909, t3912)
}
