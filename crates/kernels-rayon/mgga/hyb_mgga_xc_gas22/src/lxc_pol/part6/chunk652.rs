//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 652/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk652(t3160: f64, t675: f64, t1252: f64, t151: f64, t1248: f64, t1890: f64, t2052: f64, t39: f64) -> (f64, f64, f64, f64) {
    let t3161 = t3160 * t675;
    let t3165 = t151 * t1252;
    let t3169 = t1890 * t1248;
    let t3171 = t2052 * t39;
    (t3161, t3165, t3169, t3171)
}
