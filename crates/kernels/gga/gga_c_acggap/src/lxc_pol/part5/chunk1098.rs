//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1098/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1098<F: Float>(t1416: F, t322: F, t13298: F, t13364: F, t525: F, t13287: F, t13293: F, t1854: F, t4210: F, t1180: F, t1181: F, t13299: F, t13459: F, t13474: F, t13481: F, t13492: F, t15560: F, t17139: F, t17148: F, t17152: F, t17156: F, t1753: F, t1849: F, t3196: F, t4680: F, t5800: F) -> (F,) {
    let t22275 = t1416 * t322;
    let t22278 = t13298 * t13364 * t525 * t22275;
    let t22292 = t13293 * t13287 * t1854 * t4210;
    let t22298 = 0.42874018118069736972e-2 * t13459 + t13474 + t13481 + t13492 - 0.68598428988911579156e-2 * t22278 - 0.17149607247227894789e-2 * t1180 * t4680 * t5800 - 0.85748036236139473944e-3 * t1180 * t1181 * t15560 * t1753 - 0.85748036236139473944e-3 * t17148 - 0.68598428988911579156e-2 * t17152 + 0.68598428988911579156e-2 * t17156 - 0.34299214494455789578e-2 * t22292 + 0.34299214494455789578e-1 * t17139 * t13299 * t1849 * t3196;
    (t22298,)
}
