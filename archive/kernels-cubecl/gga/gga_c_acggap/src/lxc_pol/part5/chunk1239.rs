//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1239/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1239<F: Float>(t13087: F, t6347: F, t1165: F, t14230: F, t3456: F, t5852: F, t1163: F, t1879: F, t4210: F, t1084: F, t1173: F, t1181: F, t13597: F, t13602: F, t1531: F, t17353: F, t17355: F, t17357: F, t17362: F, t21532: F, t3196: F, t4463: F, t4540: F, t530: F, t5922: F) -> F {
    let t22652 = t13087 * t6347;
    let t22660 = t3456 * t1165 * t5852 * t14230;
    let t22680 = t1163 * t1165 * t1879 * t4210;
    let t22682 = F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t1181 * t5852 * t13597 - F::cast_from(0.32012600194825403606e-1_f64) * t22652 - F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t1165 * t5922 * t13602 - F::cast_from(0.42874018118069736972e-3_f64) * t22660 + F::cast_from(0.17149607247227894789e-1_f64) * t4463 * t1181 * t530 * t4540 - F::cast_from(0.34299214494455789578e-1_f64) * t4463 * t1181 * t21532 * t1084 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t17353 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t17355 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t17357 - F::cast_from(0.85748036236139473944e-3_f64) * t17362 + F::cast_from(0.10289764348336736874e-1_f64) * t1173 * t1165 * t1879 * t3196 + F::cast_from(0.25724410870841842184e-2_f64) * t22680;
    t22682
}
