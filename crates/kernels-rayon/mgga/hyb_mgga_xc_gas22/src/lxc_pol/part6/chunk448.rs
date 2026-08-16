//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 448/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk448(t2033: f64, t675: f64, t2002: f64, t688: f64, t708: f64, t140: f64, t1885: f64, t35: f64, t1890: f64, t704: f64, t137: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2034 = t2033 * t675;
    let t2038 = t688 * t2002;
    let t2042 = t708 * t708;
    let t2047 = 2.0_f64 / 81.0_f64 * t35 * t1885 * t140;
    let t2048 = t1890 * t704;
    let t2051 = 1.0_f64 / t697 / t137;
    (t2034, t2038, t2042, t2047, t2048, t2051)
}
