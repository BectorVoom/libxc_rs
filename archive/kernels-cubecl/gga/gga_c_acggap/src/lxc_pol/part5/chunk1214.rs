//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1214/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1214<F: Float>(t4396: F, t5755: F, t3409: F, t6396: F, t3382: F, t5612: F, t1165: F, t1173: F, t1181: F, t1531: F, t17040: F, t17042: F, t17059: F, t17064: F, t17066: F, t20972: F, t3462: F, t3463: F, t360: F, t4267: F, t4289: F, t4680: F, t5753: F, t5754: F, t5852: F) -> F {
    let t22160 = t4396 * t5755;
    let t22162 = t3409 * t6396;
    let t22174 = t3382 * t5612;
    let t22186 = F::cast_from(0.10289764348336736873e-1_f64) * t17040 + F::cast_from(0.34299214494455789578e-2_f64) * t22160 - F::cast_from(0.16006300097412701803e-1_f64) * t22162 - F::cast_from(0.68598428988911579156e-2_f64) * t3462 * t1181 * t5852 * t3463 * t360 + F::cast_from(0.34299214494455789578e-2_f64) * t1531 * t4680 * t5754 - F::cast_from(0.68598428988911579156e-2_f64) * t17042 - F::cast_from(0.17149607247227894789e-2_f64) * t17059 + F::cast_from(0.34299214494455789578e-2_f64) * t22174 - F::cast_from(0.34299214494455789578e-2_f64) * t17064 + F::cast_from(0.34299214494455789578e-2_f64) * t1531 * t1181 * t4289 * t5753 - F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t1165 * t4267 * t20972 - F::cast_from(0.24009450146119052705e-1_f64) * t17066;
    t22186
}
