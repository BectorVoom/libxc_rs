//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 756/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk756(t169: f64, t3997: f64, t4014: f64, t732: f64, t2098: f64, t736: f64, t151: f64, t4017: f64, t4019: f64, t4021: f64, t4023: f64, t4025: f64, t4027: f64, t4029: f64, t4031: f64, t4033: f64, t4035: f64, t694: f64) -> (f64, f64, f64, f64, f64) {
    let t4037 = t169 * t3997;
    let t4039 = t732 * t4014;
    let t4041 = t2098 * t3997;
    let t4043 = t736 * t4014;
    let t4045 = t151 * t3997 / 6.0_f64 - t694 * t4014 / 18.0_f64 - t4017 / 48.0_f64 + t4019 / 240.0_f64 + t4021 / 640.0_f64 - t4023 / 4480.0_f64 - t4025 / 11520.0_f64 + t4027 / 103680.0_f64 + t4029 / 258048.0_f64 - t4031 / 2838528.0_f64 - t4033 / 6881280.0_f64 + t4035 / 89456640.0_f64 + t4037 / 0.21233664e9_f64 - t4039 / 0.31850496e10_f64 - t4041 / 0.74317824e10_f64 + t4043 / 0.1263403008e12_f64;
    (t4037, t4039, t4041, t4043, t4045)
}
