//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 941/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk941<F: Float>(t2478: F, t8980: F, t1410: F, t2515: F, t2479: F, t3517: F, t7075: F, t2523: F, t3513: F, t967: F, t2521: F, t1409: F, t7150: F, t7148: F, t1433: F, t6992: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8982 = 4.0 * t2478 * t8980;
    let t8983 = t1410 * t2515;
    let t8985 = 2.0 * t2478 * t8983;
    let t8986 = t3517 * t2479;
    let t8988 = 0.96491876992155210402e2 * t7075 * t8986;
    let t8989 = t3513 * t2523;
    let t8990 = t8989 * t967;
    let t8992 = 0.32163958997385070134e2 * t2521 * t8990;
    let t8993 = t3517 * t2515;
    let t8995 = 0.16081979498692535067e2 * t2521 * t8993;
    let t8996 = t1409 * t7150;
    let t8997 = t8996 * t2479;
    let t8999 = 0.51726012919273400301e3 * t7148 * t8997;
    let t9000 = t6992 * t1433;
    (t8982, t8983, t8985, t8986, t8988, t8990, t8992, t8993, t8995, t8997, t8999, t9000)
}
