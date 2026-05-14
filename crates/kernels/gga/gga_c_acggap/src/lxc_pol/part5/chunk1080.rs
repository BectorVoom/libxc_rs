//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1080/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1080<F: Float>(t13298: F, t13364: F, t1859: F, t4210: F, t13287: F, t13293: F, t20992: F, t525: F, t5725: F, t8401: F, t12473: F, t16786: F, t16788: F, t16792: F, t16794: F, t16801: F, t16803: F, t16805: F, t17656: F, t17912: F, t1854: F, t3176: F, t398: F, t418: F) -> (F,) {
    let t21860 = t13298 * t13364 * t1859 * t4210;
    let t21864 = t13293 * t13287 * t525 * t20992;
    let t21868 = t13293 * t13364 * t8401 * t5725;
    let t21879 = 0.25724410870841842183e-2 * t16786 - 0.68598428988911579156e-2 * t16788 - 0.10289764348336736873e-1 * t17656 * t17912 * t1854 * t3176 - 0.68598428988911579156e-2 * t21860 - 0.34299214494455789578e-2 * t21864 + 0.17149607247227894789e-2 * t21868 + 0.80031500487063509016e-2 * t16792 - 0.10289764348336736873e-1 * t16794 + 0.51448821741683684367e-2 * t16801 + 0.51448821741683684367e-2 * t16803 - 0.17149607247227894789e-2 * t418 * t398 * t12473 * t1854 + 0.17149607247227894789e-1 * t16805;
    (t21879,)
}
