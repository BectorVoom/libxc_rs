//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 452/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk452(t151: f64, t154: f64, t157: f64, t160: f64, t163: f64, t166: f64, t169: f64, t2042: f64, t2070: f64, t2098: f64, t694: f64, t712: f64, t716: f64, t720: f64, t724: f64, t728: f64, t732: f64, t736: f64) -> f64 {
    let t2103 = t151 * t2042 / 6.0_f64 - t694 * t2070 / 18.0_f64 - t154 * t2042 / 48.0_f64 + t712 * t2070 / 240.0_f64 + t157 * t2042 / 640.0_f64 - t716 * t2070 / 4480.0_f64 - t160 * t2042 / 11520.0_f64 + t720 * t2070 / 103680.0_f64 + t163 * t2042 / 258048.0_f64 - t724 * t2070 / 2838528.0_f64 - t166 * t2042 / 6881280.0_f64 + t728 * t2070 / 89456640.0_f64 + t169 * t2042 / 0.21233664e9_f64 - t732 * t2070 / 0.31850496e10_f64 - t2098 * t2042 / 0.74317824e10_f64 + t736 * t2070 / 0.1263403008e12_f64;
    t2103
}
