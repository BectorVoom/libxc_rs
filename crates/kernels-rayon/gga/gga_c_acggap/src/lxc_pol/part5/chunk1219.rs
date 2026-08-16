//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1219/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1219(t1416: f64, t322: f64, t13298: f64, t13364: f64, t525: f64, t13287: f64, t13293: f64, t1854: f64, t4210: f64, t1180: f64, t1181: f64, t13299: f64, t13459: f64, t13474: f64, t13481: f64, t13492: f64, t15560: f64, t17139: f64, t17148: f64, t17152: f64, t17156: f64, t1753: f64, t1849: f64, t3196: f64, t4680: f64, t5800: f64) -> f64 {
    let t22275 = t1416 * t322;
    let t22278 = t13298 * t13364 * t525 * t22275;
    let t22292 = t13293 * t13287 * t1854 * t4210;
    let t22298 = 0.42874018118069736972e-2_f64 * t13459 + t13474 + t13481 + t13492 - 0.68598428988911579156e-2_f64 * t22278 - 0.17149607247227894789e-2_f64 * t1180 * t4680 * t5800 - 0.85748036236139473944e-3_f64 * t1180 * t1181 * t15560 * t1753 - 0.85748036236139473944e-3_f64 * t17148 - 0.68598428988911579156e-2_f64 * t17152 + 0.68598428988911579156e-2_f64 * t17156 - 0.34299214494455789578e-2_f64 * t22292 + 0.34299214494455789578e-1_f64 * t17139 * t13299 * t1849 * t3196;
    t22298
}
