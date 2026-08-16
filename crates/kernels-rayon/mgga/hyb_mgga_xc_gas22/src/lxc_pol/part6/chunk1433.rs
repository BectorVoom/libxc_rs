//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1433/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1433(t1539: f64, t7580: f64, t9503: f64, t7573: f64, t1535: f64, t11353: f64, t11335: f64, t26226: f64, t26231: f64, t26846: f64, t26850: f64, t9632: f64, t9639: f64, t9642: f64, t9654: f64, t9663: f64, t9667: f64, t9670: f64, t9678: f64, sigma2: f64) -> f64 {
    let t31069 = t1539 * t7580 * sigma2 * t9503;
    let t31074 = t1539 * t7573 * sigma2 * t9503;
    let t31083 = t1535 * t7580 * sigma2 * t9503;
    let t31086 = t11353 * t9503;
    let t31091 = t1535 * t7573 * sigma2 * t9503;
    let t31096 = t11335 * t9503;
    let t31105 = 256.0_f64 / 9.0_f64 * t9639 * t31069 + 512.0_f64 / 81.0_f64 * t9667 * t31074 + 256.0_f64 / 27.0_f64 * t9670 * t31069 + 512.0_f64 / 27.0_f64 * t9654 * t31074 - 256.0_f64 / 27.0_f64 * t9663 * t31083 - 2048.0_f64 / 729.0_f64 * t26226 * t31086 - 512.0_f64 / 81.0_f64 * t9678 * t31091 - 512.0_f64 / 27.0_f64 * t9642 * t31091 + 2048.0_f64 / 729.0_f64 * t26231 * t31096 - 256.0_f64 / 9.0_f64 * t9632 * t31083 + 2048.0_f64 / 243.0_f64 * t26850 * t31096 - 2048.0_f64 / 243.0_f64 * t26846 * t31086;
    t31105
}
