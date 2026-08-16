//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1139/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1139(t11288: f64, t1161: f64, t1129: f64, t4512: f64, t11270: f64, t2858: f64, t11274: f64, t11267: f64, t11271: f64, t11275: f64, t11279: f64, t11283: f64, t2821: f64, t2829: f64, t2834: f64, t2838: f64, t3661: f64, t3680: f64, t3688: f64, t3733: f64, t7637: f64) -> (f64, f64, f64) {
    let t11289 = t1161 * t11288;
    let t11292 = t4512 * t1129;
    let t11293 = t1161 * t11292;
    let t11296 = t2858 * t11270;
    let t11299 = t2858 * t11274;
    let t11310 = -56.0_f64 / 3.0_f64 * t7637 * t11267 - 64.0_f64 / 81.0_f64 * t3733 * t11271 + 64.0_f64 / 81.0_f64 * t3661 * t11275 + 400.0_f64 / 27.0_f64 * t3733 * t11279 + 8.0_f64 / 9.0_f64 * t2829 * t11283 + 400.0_f64 / 27.0_f64 * t3661 * t11279 + 88.0_f64 / 27.0_f64 * t2821 * t11289 - 88.0_f64 / 27.0_f64 * t2829 * t11293 - 32.0_f64 / 27.0_f64 * t2821 * t11296 + 32.0_f64 / 27.0_f64 * t2829 * t11299 - 64.0_f64 / 27.0_f64 * t3680 * t11271 + 64.0_f64 / 27.0_f64 * t3688 * t11275 - 32.0_f64 / 9.0_f64 * t2834 * t11296 + 32.0_f64 / 9.0_f64 * t2838 * t11299;
    (t11289, t11293, t11310)
}
