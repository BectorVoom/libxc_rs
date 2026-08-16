//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 755/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk755(t154: f64, t3997: f64, t4014: f64, t712: f64, t157: f64, t716: f64, t160: f64, t720: f64, t163: f64, t724: f64, t166: f64, t728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4017 = t154 * t3997;
    let t4019 = t712 * t4014;
    let t4021 = t157 * t3997;
    let t4023 = t716 * t4014;
    let t4025 = t160 * t3997;
    let t4027 = t720 * t4014;
    let t4029 = t163 * t3997;
    let t4031 = t724 * t4014;
    let t4033 = t166 * t3997;
    let t4035 = t728 * t4014;
    (t4017, t4019, t4021, t4023, t4025, t4027, t4029, t4031, t4033, t4035)
}
