//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 643/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk643(t616: f64, t82: f64, t79: f64, t1211: f64, t3068: f64, t3073: f64, t3086: f64, t3087: f64, t3093: f64, t623: f64, t627: f64, t74: f64, t81: f64) -> (f64, f64, f64) {
    let t3096 = t616 * t82;
    let t3099 = t79 * t616;
    let t3105 = -2.0_f64 * t3086 * t3087 + t623 * t3068 * t81 / 2.0_f64 + t3093 * t3087 / 4.0_f64 - 4.0_f64 * t3096 * t1211 - t3099 * t3073 - 4.0_f64 * t627 * t3068 - t74 * t3068 * t81;
    (t3096, t3099, t3105)
}
