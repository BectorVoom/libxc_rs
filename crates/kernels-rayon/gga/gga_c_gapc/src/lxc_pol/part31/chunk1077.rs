//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1077/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1077(t1: f64, t4049: f64, t172: f64, t5963: f64, t101: f64, t1645: f64, t1456: f64, t4046: f64, t115: f64, t126: f64, t442: f64, t102: f64, t1403: f64, t1593: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t13850 = t4049 * t1;
    let t13853 = t5963 * t172;
    let t14541 = t1645 * t101;
    let t14873 = 1.0_f64 / t4046 / t1456;
    let t14875 = t115 * t14873 * t126;
    let t14880 = t172 * pi * t442;
    let t14891 = t1593 * t102 * t1403;
    (t13850, t13853, t14541, t14873, t14875, t14880, t14891)
}
