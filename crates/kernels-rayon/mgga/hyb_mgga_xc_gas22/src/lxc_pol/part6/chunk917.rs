//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 917/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk917(t29: f64, t8145: f64, t125: f64, t26: f64, t3114: f64, t550: f64, t19: f64, t3118: f64, t2950: f64, t641: f64, t669: f64, t1181: f64, t1233: f64, t1824: f64, t1863: f64, t1867: f64, t1997: f64, t2949: f64, t3115: f64, t3119: f64, t547: f64, t555: f64, t558: f64, t6162: f64, t7921: f64, t7925: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8146 = t29 * t8145;
    let t8147 = t8146 * t125;
    let t8148 = t26 * t8147;
    let t8157 = t550 * t3114;
    let t8159 = t19 * t8157 / 32.0_f64;
    let t8160 = t550 * t3118;
    let t8162 = t19 * t8160 / 32.0_f64;
    let t8169 = t2950 * t641;
    let t8172 = t2950 * t669;
    let t8176 = -t555 * t558 * t7921 / 32.0_f64 - t555 * t558 * t7925 / 64.0_f64 - 3.0_f64 / 64.0_f64 * t19 * t8148 - 3.0_f64 / 64.0_f64 * t1867 * t1233 - 3.0_f64 / 32.0_f64 * t547 * t3115 - 3.0_f64 / 32.0_f64 * t547 * t3119 - t8159 - t8162 - 3.0_f64 / 64.0_f64 * t1181 * t1997 - 3.0_f64 / 32.0_f64 * t1181 * t1824 - 3.0_f64 / 64.0_f64 * t1181 * t1863 - 3.0_f64 / 16.0_f64 * t2949 * t8169 - 3.0_f64 / 16.0_f64 * t2949 * t8172 + t6162 / 144.0_f64;
    (t8147, t8148, t8157, t8159, t8160, t8162, t8169, t8172, t8176)
}
