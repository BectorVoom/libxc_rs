//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 656/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk656(t1252: f64, t154: f64, t3188: f64, t712: f64, t157: f64, t716: f64, t160: f64, t720: f64, t163: f64, t724: f64, t166: f64, t728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3191 = t154 * t1252;
    let t3194 = t712 * t3188;
    let t3196 = t157 * t1252;
    let t3199 = t716 * t3188;
    let t3201 = t160 * t1252;
    let t3204 = t720 * t3188;
    let t3206 = t163 * t1252;
    let t3209 = t724 * t3188;
    let t3211 = t166 * t1252;
    let t3214 = t728 * t3188;
    (t3191, t3194, t3196, t3199, t3201, t3204, t3206, t3209, t3211, t3214)
}
