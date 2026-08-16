//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 831/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk831(t6012: f64, t704: f64, t1890: f64, t2057: f64, t2062: f64, t2066: f64, t697: f64, t701: f64, t17: f64, t2053: f64, t700: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6279 = t6012 * t704;
    let t6281 = t1890 * t2057;
    let t6283 = t1890 * t2062;
    let t6285 = t1890 * t2066;
    let t6288 = 1.0_f64 / t697 / t701;
    let t6289 = t17 * t6288;
    let t6291 = 1.0_f64 / t2053 / t700;
    (t6279, t6281, t6283, t6285, t6288, t6289, t6291)
}
