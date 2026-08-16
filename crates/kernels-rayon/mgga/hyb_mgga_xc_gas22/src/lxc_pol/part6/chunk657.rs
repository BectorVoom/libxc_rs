//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 657/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk657(t1252: f64, t169: f64, t3188: f64, t732: f64, t2098: f64, t736: f64, t3165: f64, t3191: f64, t3194: f64, t3196: f64, t3199: f64, t3201: f64, t3204: f64, t3206: f64, t3209: f64, t3211: f64, t3214: f64, t694: f64, t708: f64) -> (f64, f64, f64) {
    let t3216 = t169 * t1252;
    let t3219 = t732 * t3188;
    let t3221 = t2098 * t1252;
    let t3224 = t736 * t3188;
    let t3226 = t3165 * t708 / 6.0_f64 - t694 * t3188 / 18.0_f64 - t3191 * t708 / 48.0_f64 + t3194 / 240.0_f64 + t3196 * t708 / 640.0_f64 - t3199 / 4480.0_f64 - t3201 * t708 / 11520.0_f64 + t3204 / 103680.0_f64 + t3206 * t708 / 258048.0_f64 - t3209 / 2838528.0_f64 - t3211 * t708 / 6881280.0_f64 + t3214 / 89456640.0_f64 + t3216 * t708 / 0.21233664e9_f64 - t3219 / 0.31850496e10_f64 - t3221 * t708 / 0.74317824e10_f64 + t3224 / 0.1263403008e12_f64;
    (t3216, t3221, t3226)
}
