//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1246/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1246<F: Float>(t1077: F, t1131: F, t1165: F, t1181: F, t13287: F, t1531: F, t15386: F, t17179: F, t17528: F, t1782: F, t22848: F, t22850: F, t22865: F, t22880: F, t22882: F, t3300: F, t360: F, t372: F, t398: F, t418: F, t5011: F, t5136: F, t5141: F, t5605: F, t5852: F, t5922: F, t8790: F) -> F {
    let t22889 = F::cast_from(0.18140473443734395377e0_f64) * t17528 + F::cast_from(0.32012600194825403606e-1_f64) * t22848 + F::cast_from(0.51448821741683684368e-2_f64) * t22850 + F::cast_from(0.25724410870841842184e-2_f64) * t418 * t398 * t3300 * t1782 * t1077 + F::cast_from(0.51448821741683684367e-2_f64) * t1531 * t1181 * t5852 * t5136 - F::cast_from(0.51448821741683684367e-2_f64) * t1531 * t1165 * t5922 * t5141 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t22865 - F::cast_from(0.68598428988911579156e-2_f64) * t17179 * t13287 * t8790 * t5605 * t360 + F::cast_from(0.10289764348336736873e-1_f64) * t17179 * t15386 * t8790 * t5605 * t372 + F::cast_from(0.68598428988911579156e-2_f64) * t22880 - F::cast_from(0.10289764348336736874e-1_f64) * t22882 - F::cast_from(0.51448821741683684368e-2_f64) * t418 * t398 * t5011 * t1782 * t1131;
    t22889
}
