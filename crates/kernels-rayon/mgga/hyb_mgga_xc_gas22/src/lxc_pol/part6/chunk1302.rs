//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1302/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1302(t143: f64, t28223: f64, t28242: f64, t28274: f64, t28309: f64, t10331: f64, t151: f64, t154: f64, t157: f64, t160: f64, t163: f64, t166: f64, t169: f64, t2098: f64, t28162: f64, t694: f64, t708: f64, t712: f64, t716: f64, t720: f64, t724: f64, t728: f64, t732: f64, t736: f64) -> (f64, f64) {
    let t145 = 0.135e1_f64 < t143;
    let t28311 = t28223 + t28242 + t28274 + t28309;
    let t28312 = piecewise3(t145, t28311, 0.0_f64);
    let t28335 = -t166 * t28162 / 3440640.0_f64 + t169 * t28162 / 0.10616832e9_f64 - t2098 * t28162 / 0.37158912e10_f64 + t151 * t28162 / 3.0_f64 - t154 * t28162 / 24.0_f64 + t157 * t28162 / 320.0_f64 - t160 * t28162 / 5760.0_f64 + t163 * t28162 / 129024.0_f64 - t694 * t28312 / 18.0_f64 + t712 * t28312 / 240.0_f64 - t716 * t28312 / 4480.0_f64 + t720 * t28312 / 103680.0_f64 - t724 * t28312 / 2838528.0_f64 + t728 * t28312 / 89456640.0_f64 - t732 * t28312 / 0.31850496e10_f64 + t736 * t28312 / 0.1263403008e12_f64 - t154 * t10331 * t708 / 24.0_f64 + t157 * t10331 * t708 / 320.0_f64;
    (t28311, t28335)
}
