//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1168/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1168(t1175: f64, t1410: f64, t1460: f64, t322: f64, t1181: f64, t3391: f64, t6337: f64, t1165: f64, t1173: f64, t1180: f64, t1532: f64, t16123: f64, t16125: f64, t16127: f64, t16141: f64, t16143: f64, t16160: f64, t20545: f64, t3176: f64, t372: f64, t4261: f64, t4262: f64, t4298: f64, t5275: f64, t5606: f64, t5922: f64, t6218: f64) -> (f64, f64) {
    let t21099 = t1175 * t1410;
    let t21118 = t1460 * t322;
    let t21121 = t3391 * t1181 * t6337 * t21118;
    let t21127 = -0.34299214494455789578e-2_f64 * t1173 * t1165 * t5922 * t5275 + 0.34299214494455789578e-2_f64 * t1173 * t1165 * t1532 * t21099 - 0.34299214494455789577e-2_f64 * t16123 + 0.17149607247227894789e-2_f64 * t16125 + 0.16006300097412701803e-1_f64 * t16127 + 0.12004725073059526352e-1_f64 * t16141 + 0.32012600194825403606e-1_f64 * t16143 - t4261 * t4262 * t6218 * t372 / 12.0_f64 - 0.13719685797782315831e-1_f64 * t16160 + 0.10289764348336736874e-1_f64 * t1180 * t1165 * t20545 * t3176 - 0.10289764348336736873e-1_f64 * t21121 + 0.17149607247227894789e-2_f64 * t1180 * t1165 * t4298 * t5606;
    (t21118, t21127)
}
