//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1150/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1150<F: Float>(t1439: F, t360: F, t1165: F, t14368: F, t1884: F, t4210: F, t1416: F, t372: F, t1345: F, t322: F, t13298: F, t13299: F, t525: F, t1017: F, t1089: F, t1096: F, t1181: F, t13286: F, t13287: F, t13364: F, t14177: F, t14181: F, t1459: F, t1734: F, t1795: F, t18368: F, t22099: F, t368: F, t398: F, t418: F, t4267: F, t4463: F, t5011: F, t5506: F, t5710: F, t8401: F) -> (F, F) {
    let t23718 = t1439 * t360;
    let t23725 = t14368 * t1165 * t1884 * t4210;
    let t23736 = t1416 * t372;
    let t23745 = t1345 * t322;
    let t23748 = t13298 * t13299 * t525 * t23745;
    let t23750 = -7.0 / 72.0 * t18368 - 0.34299214494455789578e-2 * t14177 - 0.17149607247227894789e-2 * t14181 - 0.51448821741683684368e-2 * t418 * t398 * t5011 * t1795 * t1017 - 0.34299214494455789578e-2 * t418 * t1089 * t368 * t5506 * t360 - 0.68598428988911579156e-1 * t4463 * t1181 * t4267 * t23718 + 0.85748036236139473944e-2 * t23725 + 0.68598428988911579156e-2 * t418 * t1089 * t22099 * t1096 - 0.51448821741683684367e-2 * t418 * t1089 * t1459 * t1734 * t1017 - 0.13719685797782315831e-1 * t13286 * t13287 * t525 * t23736 + 0.68598428988911579156e-2 * t13286 * t13364 * t8401 * t5710 + 0.68598428988911579156e-2 * t23748;
    (t23718, t23750)
}
