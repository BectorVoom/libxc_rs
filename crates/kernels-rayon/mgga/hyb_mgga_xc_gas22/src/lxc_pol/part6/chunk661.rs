//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 661/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk661(t181: f64, t740: f64, t178: f64, t1270: f64, t173: f64, t180: f64, t3227: f64, t3232: f64, t3245: f64, t3246: f64, t3252: f64, t747: f64, t751: f64) -> (f64, f64, f64) {
    let t3255 = t740 * t181;
    let t3258 = t178 * t740;
    let t3264 = -2.0_f64 * t3245 * t3246 + t747 * t3227 * t180 / 2.0_f64 + t3252 * t3246 / 4.0_f64 - 4.0_f64 * t3255 * t1270 - t3258 * t3232 - 4.0_f64 * t751 * t3227 - t173 * t3227 * t180;
    (t3255, t3258, t3264)
}
