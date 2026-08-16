//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1201/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1201(t13298: f64, t13364: f64, t1859: f64, t4210: f64, t13287: f64, t13293: f64, t20992: f64, t525: f64, t5725: f64, t8401: f64, t12473: f64, t16786: f64, t16788: f64, t16792: f64, t16794: f64, t16801: f64, t16803: f64, t16805: f64, t17656: f64, t17912: f64, t1854: f64, t3176: f64, t398: f64, t418: f64) -> f64 {
    let t21860 = t13298 * t13364 * t1859 * t4210;
    let t21864 = t13293 * t13287 * t525 * t20992;
    let t21868 = t13293 * t13364 * t8401 * t5725;
    let t21879 = 0.25724410870841842183e-2_f64 * t16786 - 0.68598428988911579156e-2_f64 * t16788 - 0.10289764348336736873e-1_f64 * t17656 * t17912 * t1854 * t3176 - 0.68598428988911579156e-2_f64 * t21860 - 0.34299214494455789578e-2_f64 * t21864 + 0.17149607247227894789e-2_f64 * t21868 + 0.80031500487063509016e-2_f64 * t16792 - 0.10289764348336736873e-1_f64 * t16794 + 0.51448821741683684367e-2_f64 * t16801 + 0.51448821741683684367e-2_f64 * t16803 - 0.17149607247227894789e-2_f64 * t418 * t398 * t12473 * t1854 + 0.17149607247227894789e-1_f64 * t16805;
    t21879
}
