//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1246/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1246(t1077: f64, t1131: f64, t1165: f64, t1181: f64, t13287: f64, t1531: f64, t15386: f64, t17179: f64, t17528: f64, t1782: f64, t22848: f64, t22850: f64, t22865: f64, t22880: f64, t22882: f64, t3300: f64, t360: f64, t372: f64, t398: f64, t418: f64, t5011: f64, t5136: f64, t5141: f64, t5605: f64, t5852: f64, t5922: f64, t8790: f64) -> f64 {
    let t22889 = 0.18140473443734395377e0_f64 * t17528 + 0.32012600194825403606e-1_f64 * t22848 + 0.51448821741683684368e-2_f64 * t22850 + 0.25724410870841842184e-2_f64 * t418 * t398 * t3300 * t1782 * t1077 + 0.51448821741683684367e-2_f64 * t1531 * t1181 * t5852 * t5136 - 0.51448821741683684367e-2_f64 * t1531 * t1165 * t5922 * t5141 + 7.0_f64 / 144.0_f64 * t22865 - 0.68598428988911579156e-2_f64 * t17179 * t13287 * t8790 * t5605 * t360 + 0.10289764348336736873e-1_f64 * t17179 * t15386 * t8790 * t5605 * t372 + 0.68598428988911579156e-2_f64 * t22880 - 0.10289764348336736874e-1_f64 * t22882 - 0.51448821741683684368e-2_f64 * t418 * t398 * t5011 * t1782 * t1131;
    t22889
}
