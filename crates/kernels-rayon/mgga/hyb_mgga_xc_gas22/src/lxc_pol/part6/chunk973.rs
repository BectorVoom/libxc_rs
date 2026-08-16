//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 973/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk973(t1410: f64, t2479: f64, t2521: f64, t3482: f64, t6951: f64, t3518: f64, t7070: f64, t3514: f64, t967: f64, t2478: f64, t2515: f64, t3517: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8973 = t1410 * t2479;
    let t8975 = 6.0_f64 * t2521 * t8973;
    let t8977 = 4.0_f64 * t6951 * t3482;
    let t8979 = 0.32163958997385070134e2_f64 * t7070 * t3518;
    let t8980 = t3514 * t967;
    let t8982 = 4.0_f64 * t2478 * t8980;
    let t8983 = t1410 * t2515;
    let t8985 = 2.0_f64 * t2478 * t8983;
    let t8986 = t3517 * t2479;
    (t8973, t8975, t8977, t8979, t8980, t8982, t8983, t8985, t8986)
}
