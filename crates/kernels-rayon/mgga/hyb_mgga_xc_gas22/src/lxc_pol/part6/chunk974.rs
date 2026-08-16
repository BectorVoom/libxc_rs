//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 974/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk974(t7075: f64, t8986: f64, t2523: f64, t3513: f64, t967: f64, t2521: f64, t2515: f64, t3517: f64, t1409: f64, t7150: f64, t2479: f64, t7148: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8988 = 0.96491876992155210402e2_f64 * t7075 * t8986;
    let t8989 = t3513 * t2523;
    let t8990 = t8989 * t967;
    let t8992 = 0.32163958997385070134e2_f64 * t2521 * t8990;
    let t8993 = t3517 * t2515;
    let t8995 = 0.16081979498692535067e2_f64 * t2521 * t8993;
    let t8996 = t1409 * t7150;
    let t8997 = t8996 * t2479;
    let t8999 = 0.51726012919273400301e3_f64 * t7148 * t8997;
    (t8988, t8990, t8992, t8993, t8995, t8997, t8999)
}
