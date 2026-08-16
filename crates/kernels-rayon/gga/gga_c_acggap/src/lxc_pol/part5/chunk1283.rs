//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1283/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1283(t1017: f64, t1089: f64, t1096: f64, t1181: f64, t13286: f64, t13287: f64, t13364: f64, t14177: f64, t14181: f64, t1459: f64, t1734: f64, t1795: f64, t18368: f64, t22099: f64, t23718: f64, t23725: f64, t23736: f64, t23748: f64, t360: f64, t368: f64, t398: f64, t418: f64, t4267: f64, t4463: f64, t5011: f64, t525: f64, t5506: f64, t5710: f64, t8401: f64) -> f64 {
    let t23750 = -7.0_f64 / 72.0_f64 * t18368 - 0.34299214494455789578e-2_f64 * t14177 - 0.17149607247227894789e-2_f64 * t14181 - 0.51448821741683684368e-2_f64 * t418 * t398 * t5011 * t1795 * t1017 - 0.34299214494455789578e-2_f64 * t418 * t1089 * t368 * t5506 * t360 - 0.68598428988911579156e-1_f64 * t4463 * t1181 * t4267 * t23718 + 0.85748036236139473944e-2_f64 * t23725 + 0.68598428988911579156e-2_f64 * t418 * t1089 * t22099 * t1096 - 0.51448821741683684367e-2_f64 * t418 * t1089 * t1459 * t1734 * t1017 - 0.13719685797782315831e-1_f64 * t13286 * t13287 * t525 * t23736 + 0.68598428988911579156e-2_f64 * t13286 * t13364 * t8401 * t5710 + 0.68598428988911579156e-2_f64 * t23748;
    t23750
}
