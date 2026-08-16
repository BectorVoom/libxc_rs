//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1239/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1239(t13087: f64, t6347: f64, t1165: f64, t14230: f64, t3456: f64, t5852: f64, t1163: f64, t1879: f64, t4210: f64, t1084: f64, t1173: f64, t1181: f64, t13597: f64, t13602: f64, t1531: f64, t17353: f64, t17355: f64, t17357: f64, t17362: f64, t21532: f64, t3196: f64, t4463: f64, t4540: f64, t530: f64, t5922: f64) -> f64 {
    let t22652 = t13087 * t6347;
    let t22660 = t3456 * t1165 * t5852 * t14230;
    let t22680 = t1163 * t1165 * t1879 * t4210;
    let t22682 = 0.85748036236139473944e-3_f64 * t1531 * t1181 * t5852 * t13597 - 0.32012600194825403606e-1_f64 * t22652 - 0.85748036236139473944e-3_f64 * t1531 * t1165 * t5922 * t13602 - 0.42874018118069736972e-3_f64 * t22660 + 0.17149607247227894789e-1_f64 * t4463 * t1181 * t530 * t4540 - 0.34299214494455789578e-1_f64 * t4463 * t1181 * t21532 * t1084 - 7.0_f64 / 12.0_f64 * t17353 - 7.0_f64 / 12.0_f64 * t17355 - 7.0_f64 / 24.0_f64 * t17357 - 0.85748036236139473944e-3_f64 * t17362 + 0.10289764348336736874e-1_f64 * t1173 * t1165 * t1879 * t3196 + 0.25724410870841842184e-2_f64 * t22680;
    t22682
}
