//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 887/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk887(t458: f64, t7543: f64, t496: f64, t1052: f64, t2814: f64, t2640: f64, t2643: f64, t2676: f64, t1089: f64, t1096: f64, t7410: f64, t1110: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7544 = t458 * t7543;
    let t7546 = 120.0_f64 * t7544 * t496;
    let t7547 = t1052 * t2814;
    let t7549 = t2643 * t2640;
    let t7551 = t2643 * t2676;
    let t7554 = t1089 * t7410 * t1096;
    let t7556 = 0.5848223622634646207e0_f64 * t1110 * t7554;
    (t7544, t7546, t7547, t7549, t7551, t7554, t7556)
}
