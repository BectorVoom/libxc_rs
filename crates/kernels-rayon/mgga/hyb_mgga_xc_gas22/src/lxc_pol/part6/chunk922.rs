//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 922/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk922(t166: f64, t3188: f64, t169: f64, t2098: f64, t151: f64, t2070: f64, t3165: f64, t3191: f64, t3196: f64, t3201: f64, t3206: f64, t3211: f64, t3216: f64, t3221: f64, t708: f64, t8226: f64, t8231: f64, t8236: f64, t8241: f64) -> f64 {
    let t8246 = t166 * t3188;
    let t8251 = t169 * t3188;
    let t8256 = t2098 * t3188;
    let t8261 = t151 * t3188;
    let t8266 = -t8226 * t708 / 24.0_f64 - t3191 * t2070 / 48.0_f64 + t8231 * t708 / 320.0_f64 + t3196 * t2070 / 640.0_f64 - t8236 * t708 / 5760.0_f64 - t3201 * t2070 / 11520.0_f64 + t8241 * t708 / 129024.0_f64 + t3206 * t2070 / 258048.0_f64 - t8246 * t708 / 3440640.0_f64 - t3211 * t2070 / 6881280.0_f64 + t8251 * t708 / 0.10616832e9_f64 + t3216 * t2070 / 0.21233664e9_f64 - t8256 * t708 / 0.37158912e10_f64 - t3221 * t2070 / 0.74317824e10_f64 + t8261 * t708 / 3.0_f64 + t3165 * t2070 / 6.0_f64;
    t8266
}
