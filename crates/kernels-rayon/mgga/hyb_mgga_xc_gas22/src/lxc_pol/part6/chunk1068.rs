//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1068/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1068(t3997: f64, t720: f64, t157: f64, t4014: f64, t724: f64, t160: f64, t728: f64, t1890: f64, t4002: f64, t3925: f64, t6291: f64, t675: f64, t8296: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10262 = t720 * t3997;
    let t10267 = t157 * t4014;
    let t10270 = t724 * t3997;
    let t10275 = t160 * t4014;
    let t10278 = t728 * t3997;
    let t10286 = t1890 * t4002;
    let t10288 = t6291 * t3925;
    let t10290 = t8296 * t10288 * t675;
    (t10262, t10267, t10270, t10275, t10278, t10286, t10288, t10290)
}
