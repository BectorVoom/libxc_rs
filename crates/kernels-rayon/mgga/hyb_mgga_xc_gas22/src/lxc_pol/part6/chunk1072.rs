//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1072/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1072(t143: f64, t10330: f64, t10262: f64, t10267: f64, t10270: f64, t10275: f64, t10278: f64, t3188: f64, t3196: f64, t3201: f64, t3206: f64, t694: f64, t708: f64, t712: f64, t716: f64, t720: f64, t724: f64, t728: f64, t732: f64, t736: f64) -> (f64, f64) {
    let t145 = 0.135e1_f64 < t143;
    let t10331 = piecewise3(t145, t10330, 0.0_f64);
    let t10348 = -t10262 * t708 / 80.0_f64 + t3196 * t3188 / 320.0_f64 + t10267 * t708 / 640.0_f64 + t10270 * t708 / 1152.0_f64 - t3201 * t3188 / 5760.0_f64 - t10275 * t708 / 11520.0_f64 - t10278 * t708 / 21504.0_f64 + t3206 * t3188 / 129024.0_f64 - t694 * t10331 / 18.0_f64 + t712 * t10331 / 240.0_f64 - t716 * t10331 / 4480.0_f64 + t720 * t10331 / 103680.0_f64 - t724 * t10331 / 2838528.0_f64 + t728 * t10331 / 89456640.0_f64 - t732 * t10331 / 0.31850496e10_f64 + t736 * t10331 / 0.1263403008e12_f64;
    (t10331, t10348)
}
