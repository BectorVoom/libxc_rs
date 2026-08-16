//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 988/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk988(t1179: f64, t12727: f64, t12729: f64, t15874: f64, t18055: f64, t18059: f64, t18062: f64, t18066: f64, t18069: f64, t18072: f64, t18076: f64, t3244: f64, t4457: f64, t4464: f64, t9093: f64, t9102: f64, t9116: f64, t9122: f64) -> f64 {
    let t18080 = -0.19318136643975017455e-1_f64 * t12727 - 0.33587136305576131526e-2_f64 * t12729 + t9093 - 0.13186481011862155443e4_f64 * t4464 * t18055 + 0.34014423178468276541e6_f64 * t9116 * t18059 - 0.34014423178468276541e6_f64 * t9122 * t18062 + 0.26372962023724310886e4_f64 * t4457 * t18066 + 0.56690705297447127569e5_f64 * t9102 * t18069 + 0.15146801702008125515e1_f64 * t3244 * t18072 + 0.25190352229182098644e-1_f64 * t1179 * t18076 + 0.75734008510040627575e0_f64 * t15874;
    t18080
}
