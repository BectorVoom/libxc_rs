//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1360/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1360(t11028: f64, t6951: f64, t11032: f64, t7070: f64, t11068: f64, t11111: f64, t11117: f64, t21628: f64, t2538: f64, t25556: f64, t2560: f64, t25643: f64, t25661: f64, t25810: f64, t25813: f64, t25816: f64, t29602: f64, t29627: f64, t29629: f64, t29631: f64, t29633: f64, t29635: f64, t3532: f64, t3551: f64, t7159: f64, t9048: f64, t9058: f64, t9073: f64, t9083: f64, t9210: f64, t9245: f64, t986: f64) -> (f64, f64, f64) {
    let t29637 = 4.0_f64 * t6951 * t11028;
    let t29639 = 0.32163958997385070134e2_f64 * t7070 * t11032;
    let t29640 = -4.0_f64 * t2538 * t11068 * t986 + 0.64327917994770140268e2_f64 * t2560 * t29602 * t986 + 0.12865583598954028054e3_f64 * t7159 * t11111 + 0.4138081033541872024e4_f64 * t21628 * t11117 - 8.0_f64 * t25643 * t3532 + 0.12865583598954028054e3_f64 * t25556 * t3551 - 8.0_f64 * t9245 * t9073 + 0.12865583598954028054e3_f64 * t9210 * t9083 - 0.4155806185363551302e3_f64 * t25816 * t9058 + 24.0_f64 * t25810 * t9073 - 0.77193501593724168323e3_f64 * t25661 * t9083 + 0.14035736694323150897e2_f64 * t25813 * t9048 + t29627 - t29629 - t29631 + t29633 + t29635 + t29637 - t29639;
    (t29637, t29639, t29640)
}
