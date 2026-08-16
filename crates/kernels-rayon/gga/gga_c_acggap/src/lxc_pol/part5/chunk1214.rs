//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1214/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1214(t4396: f64, t5755: f64, t3409: f64, t6396: f64, t3382: f64, t5612: f64, t1165: f64, t1173: f64, t1181: f64, t1531: f64, t17040: f64, t17042: f64, t17059: f64, t17064: f64, t17066: f64, t20972: f64, t3462: f64, t3463: f64, t360: f64, t4267: f64, t4289: f64, t4680: f64, t5753: f64, t5754: f64, t5852: f64) -> f64 {
    let t22160 = t4396 * t5755;
    let t22162 = t3409 * t6396;
    let t22174 = t3382 * t5612;
    let t22186 = 0.10289764348336736873e-1_f64 * t17040 + 0.34299214494455789578e-2_f64 * t22160 - 0.16006300097412701803e-1_f64 * t22162 - 0.68598428988911579156e-2_f64 * t3462 * t1181 * t5852 * t3463 * t360 + 0.34299214494455789578e-2_f64 * t1531 * t4680 * t5754 - 0.68598428988911579156e-2_f64 * t17042 - 0.17149607247227894789e-2_f64 * t17059 + 0.34299214494455789578e-2_f64 * t22174 - 0.34299214494455789578e-2_f64 * t17064 + 0.34299214494455789578e-2_f64 * t1531 * t1181 * t4289 * t5753 - 0.68598428988911579156e-2_f64 * t1173 * t1165 * t4267 * t20972 - 0.24009450146119052705e-1_f64 * t17066;
    t22186
}
