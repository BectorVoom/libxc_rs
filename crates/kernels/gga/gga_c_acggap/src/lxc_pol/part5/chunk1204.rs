//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1204/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1204<F: Float>(t12930: F, t1755: F, t13092: F, t6140: F, t1165: F, t1173: F, t1180: F, t1181: F, t16863: F, t16865: F, t16867: F, t16869: F, t16871: F, t16893: F, t16897: F, t20433: F, t21128: F, t3396: F, t3462: F, t4267: F, t4516: F, t4643: F, t5106: F, t5150: F, t530: F, t5852: F) -> F {
    let t21932 = t12930 * t1755;
    let t21944 = t13092 * t6140;
    let t21950 = -F::new(0.10289764348336736874e-1) * t3462 * t1165 * t5852 * t5150 - F::new(0.34299214494455789578e-2) * t1173 * t1165 * t4267 * t20433 + F::new(0.32012600194825403606e-1) * t16863 - F::new(0.16006300097412701803e-1) * t16865 + F::new(0.90702367218671976886e-1) * t16867 + F::new(0.34013387707001991332e-1) * t16869 + F::new(0.80031500487063509014e-2) * t21932 + F::new(0.17149607247227894789e-2) * t1180 * t1181 * t4643 * t21128 + F::new(0.17149607247227894789e-2) * t16893 + F::new(0.34299214494455789578e-2) * t16897 + F::new(0.68598428988911579156e-2) * t3396 * t1181 * t4267 * t4516 + F::new(0.96037800584476210816e-1) * t21944 - F::new(0.10289764348336736873e0) * t16871 * t1181 * t530 * t5106;
    t21950
}
