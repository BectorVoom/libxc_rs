//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1228/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1228(t1890: f64, t8299: f64, t19746: f64, t35: f64, t8330: f64, t2054: f64, t8329: f64, t1238: f64, t675: f64, t2051: f64, t39: f64, t6299: f64) -> (f64, f64, f64, f64, f64) {
    let t24137 = t1890 * t8299;
    let t24140 = t35 * t19746 * t8330;
    let t24142 = t8329 * t2054;
    let t24143 = t1238 * t675;
    let t24149 = t2051 * t39 * t6299;
    (t24137, t24140, t24142, t24143, t24149)
}
