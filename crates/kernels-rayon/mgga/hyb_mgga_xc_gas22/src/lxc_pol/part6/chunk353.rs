//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 353/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk353(t143: f64, t1238: f64, t688: f64, t699: f64, t702: f64, t572: f64, t696: f64) -> (f64, f64, f64, f64) {
    let t145 = 0.135e1_f64 < t143;
    let t1243 = t688 * t1238;
    let t1248 = t699 * t702 * t1238;
    let t1251 = -t572 * t1248 / 54.0_f64 - t696 / 54.0_f64;
    let t1252 = piecewise3(t145, t1251, 0.0_f64);
    (t1243, t1248, t1251, t1252)
}
