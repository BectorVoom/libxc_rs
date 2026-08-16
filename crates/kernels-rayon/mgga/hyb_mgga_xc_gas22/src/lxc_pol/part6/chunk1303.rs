//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1303/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1303(t10331: f64, t160: f64, t20396: f64, t2042: f64, t2098: f64, t3997: f64, t4014: f64, t4017: f64, t4019: f64, t4021: f64, t4023: f64, t4025: f64, t4027: f64, t4029: f64, t4031: f64, t4033: f64, t4035: f64, t4037: f64, t4039: f64, t4041: f64, t4043: f64, t6270: f64, t708: f64) -> f64 {
    let t28376 = -t160 * t10331 * t708 / 5760.0_f64 - t4035 * t2042 / 21504.0_f64 - t4037 * t2042 / 32768.0_f64 + t4039 * t2042 / 491520.0_f64 + 17.0_f64 / 13271040.0_f64 * t4041 * t2042 - t4043 * t2042 / 13271040.0_f64 - 19.0_f64 / 412876800.0_f64 * t20396 * t3997 * t2042 + t6270 * t4014 * t2042 / 412876800.0_f64 + 10.0_f64 / 3.0_f64 * t4017 * t2042 - 2.0_f64 / 3.0_f64 * t4019 * t2042 - 7.0_f64 / 8.0_f64 * t4021 * t2042 + t4023 * t2042 / 8.0_f64 + 9.0_f64 / 80.0_f64 * t4025 * t2042 - t4027 * t2042 / 80.0_f64 - 11.0_f64 / 1152.0_f64 * t4029 * t2042 + t4031 * t2042 / 1152.0_f64 + 13.0_f64 / 21504.0_f64 * t4033 * t2042 - t2098 * t10331 * t708 / 0.37158912e10_f64;
    t28376
}
