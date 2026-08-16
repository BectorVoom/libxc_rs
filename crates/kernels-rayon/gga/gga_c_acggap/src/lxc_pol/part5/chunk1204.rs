//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1204/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1204(t12930: f64, t1755: f64, t13092: f64, t6140: f64, t1165: f64, t1173: f64, t1180: f64, t1181: f64, t16863: f64, t16865: f64, t16867: f64, t16869: f64, t16871: f64, t16893: f64, t16897: f64, t20433: f64, t21128: f64, t3396: f64, t3462: f64, t4267: f64, t4516: f64, t4643: f64, t5106: f64, t5150: f64, t530: f64, t5852: f64) -> f64 {
    let t21932 = t12930 * t1755;
    let t21944 = t13092 * t6140;
    let t21950 = -0.10289764348336736874e-1_f64 * t3462 * t1165 * t5852 * t5150 - 0.34299214494455789578e-2_f64 * t1173 * t1165 * t4267 * t20433 + 0.32012600194825403606e-1_f64 * t16863 - 0.16006300097412701803e-1_f64 * t16865 + 0.90702367218671976886e-1_f64 * t16867 + 0.34013387707001991332e-1_f64 * t16869 + 0.80031500487063509014e-2_f64 * t21932 + 0.17149607247227894789e-2_f64 * t1180 * t1181 * t4643 * t21128 + 0.17149607247227894789e-2_f64 * t16893 + 0.34299214494455789578e-2_f64 * t16897 + 0.68598428988911579156e-2_f64 * t3396 * t1181 * t4267 * t4516 + 0.96037800584476210816e-1_f64 * t21944 - 0.10289764348336736873e0_f64 * t16871 * t1181 * t530 * t5106;
    t21950
}
