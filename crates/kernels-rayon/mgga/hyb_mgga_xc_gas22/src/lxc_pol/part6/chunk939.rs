//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 939/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk939(t2026: f64, t2212: f64, t191: f64, t214: f64, t3: f64, t675: f64, t13: f64, t2969: f64, t6429: f64) -> (f64, f64, f64, f64) {
    let t8518 = t2212 * t2026;
    let t8519 = t8518 * t191;
    let t8520 = t214 * t3;
    let t8521 = t8520 * t675;
    let t8526 = t6429 * t13 * t2969;
    (t8518, t8519, t8521, t8526)
}
